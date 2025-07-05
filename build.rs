use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    keywords: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    og_title: Option<String>,
    #[serde(default)]
    og_description: Option<String>,
    #[serde(default)]
    og_type: Option<String>,
    #[serde(default)]
    og_url: Option<String>,
    #[serde(default)]
    og_site_name: Option<String>,
    #[serde(default)]
    twitter_card: Option<String>,
    #[serde(default)]
    twitter_title: Option<String>,
    #[serde(default)]
    twitter_description: Option<String>,
    #[serde(default)]
    canonical_url: Option<String>,
}



// Declarative structure definition system
#[derive(Debug, Clone)]
enum FieldType {
    StaticStr,
    StaticStrSlice,
}

#[derive(Debug, Clone)]
struct FieldDef {
    name: &'static str,
    field_type: FieldType,
    doc: Option<&'static str>,
}

#[derive(Debug, Clone)]
struct StructDef {
    name: &'static str,
    derives: Vec<&'static str>,
    fields: Vec<FieldDef>,
}

impl FieldDef {
    fn new(name: &'static str, field_type: FieldType) -> Self {
        Self {
            name,
            field_type,
            doc: None,
        }
    }

    fn with_doc(mut self, doc: &'static str) -> Self {
        self.doc = Some(doc);
        self
    }

    fn rust_type(&self) -> &'static str {
        match self.field_type {
            FieldType::StaticStr => "&'static str",
            FieldType::StaticStrSlice => "&'static [&'static str]",
        }
    }

    fn generate_field_declaration(&self) -> String {
        let doc = self
            .doc
            .map(|d| format!("    /// {}\n", d))
            .unwrap_or_default();
        format!("{}    pub {}: {},", doc, self.name, self.rust_type())
    }
}

impl StructDef {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            derives: vec!["Clone", "Debug", "PartialEq"],
            fields: Vec::new(),
        }
    }

    fn add_field(mut self, field: FieldDef) -> Self {
        self.fields.push(field);
        self
    }

    #[allow(unused)]
    fn with_derives(mut self, derives: Vec<&'static str>) -> Self {
        self.derives = derives;
        self
    }

    fn generate_struct_definition(&self) -> String {
        let derives = self.derives.join(", ");
        let fields = self
            .fields
            .iter()
            .map(|f| f.generate_field_declaration())
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "#[derive({})]\npub struct {} {{\n{}\n}}",
            derives, self.name, fields
        )
    }
}

// Data container for actual post values
#[derive(Debug, Clone)]
struct PostData {
    values: std::collections::HashMap<String, PostValue>,
}

#[derive(Debug, Clone)]
enum PostValue {
    Str(String),
    StrSlice(Vec<String>),
}

impl PostData {
    fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    fn set_str<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.values.insert(key.into(), PostValue::Str(value.into()));
        self
    }

    fn set_str_slice<K: Into<String>>(mut self, key: K, values: Vec<String>) -> Self {
        self.values.insert(key.into(), PostValue::StrSlice(values));
        self
    }

    fn escape_rust_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    fn generate_instance(&self, struct_def: &StructDef) -> String {
        let fields = struct_def
            .fields
            .iter()
            .map(|field| {
                let value = match &self.values[field.name] {
                    PostValue::Str(s) => format!("\"{}\"", Self::escape_rust_string(s)),
                    PostValue::StrSlice(slice) => {
                        let items = slice
                            .iter()
                            .map(|s| format!("\"{}\"", Self::escape_rust_string(s)))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("&[{}]", items)
                    }
                };
                format!("        {}: {},", field.name, value)
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!("    {} {{\n{}\n    }}", struct_def.name, fields)
    }
}

// Code generation orchestrator
struct CodeGen {
    struct_defs: Vec<StructDef>,
    static_arrays: Vec<(String, String, Vec<PostData>)>, // (name, element_type, data)
}

impl CodeGen {
    fn new() -> Self {
        Self {
            struct_defs: Vec::new(),
            static_arrays: Vec::new(),
        }
    }

    fn add_struct(mut self, struct_def: StructDef) -> Self {
        self.struct_defs.push(struct_def);
        self
    }

    fn add_static_array<N: Into<String>, T: Into<String>>(
        mut self,
        name: N,
        element_type: T,
        data: Vec<PostData>,
    ) -> Self {
        self.static_arrays
            .push((name.into(), element_type.into(), data));
        self
    }

    fn generate_code(&self) -> String {
        let mut output = String::new();

        // Generate struct definitions
        for struct_def in &self.struct_defs {
            output.push_str(&struct_def.generate_struct_definition());
            output.push_str("\n\n");
        }

        // Generate static arrays
        for (name, element_type, data) in &self.static_arrays {
            let instances = data
                .iter()
                .map(|post_data| {
                    // Find the struct definition for this element type
                    let struct_def = self
                        .struct_defs
                        .iter()
                        .find(|s| s.name == element_type)
                        .expect("Element type must have corresponding struct definition");
                    post_data.generate_instance(struct_def)
                })
                .collect::<Vec<_>>()
                .join(",\n");

            output.push_str(&format!(
                "static {}: &[{}] = &[\n{}\n];\n",
                name, element_type, instances
            ));
        }

        output
    }

    fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        write!(file, "{}", self.generate_code())
    }
}

fn process_markdown_file(path: &Path) -> Result<PostData, Box<dyn std::error::Error>> {
    let slug = path
        .file_stem()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();

    let content = fs::read_to_string(path)?;
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&content);

    let frontmatter: Frontmatter = result
        .data
        .as_ref()
        .ok_or("No frontmatter found")?
        .deserialize()?;

    let markdown_content = result.content;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(PostData::new()
        .set_str("slug", slug)
        .set_str("title", frontmatter.title)
        .set_str("date", frontmatter.date)
        .set_str("summary", frontmatter.summary.unwrap_or_default())
        .set_str("body", html_output)
        .set_str_slice("tags", frontmatter.tags)
        .set_str("raw_content", markdown_content))
}

fn generate_html_file(post_data: &PostData, base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let slug = match &post_data.values["slug"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid slug".into()),
    };
    
    let title = match &post_data.values["title"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid title".into()),
    };
    
    let date = match &post_data.values["date"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid date".into()),
    };
    
    let summary = match &post_data.values["summary"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid summary".into()),
    };
    
    let body = match &post_data.values["body"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid body".into()),
    };
    
    let tags = match &post_data.values["tags"] {
        PostValue::StrSlice(tags) => tags,
        _ => return Err("Invalid tags".into()),
    };

    let tags_html = tags.iter()
        .map(|tag| format!("<span class=\"tag\">{}</span>", html_escape(tag)))
        .collect::<Vec<_>>()
        .join("");

    let canonical_url = format!("{}/blog/{}", base_url, slug);
    
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Hex</title>
    <meta name="description" content="{}">
    <meta name="author" content="Vladyslav 'Hex' Yamkovyi">
    <meta name="keywords" content="{}">
    
    <!-- Open Graph -->
    <meta property="og:title" content="{} - Hex">
    <meta property="og:description" content="{}">
    <meta property="og:type" content="article">
    <meta property="og:url" content="{}">
    <meta property="og:site_name" content="Hex's Blog">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="{} - Hex">
    <meta name="twitter:description" content="{}">
    
    <!-- Canonical URL -->
    <link rel="canonical" href="{}">
    
    <!-- Structured Data -->
    <script type="application/ld+json">
    {{
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": "{}",
        "description": "{}",
        "author": {{
            "@type": "Person",
            "name": "Vladyslav 'Hex' Yamkovyi",
            "url": "{}"
        }},
        "datePublished": "{}",
        "url": "{}",
        "mainEntityOfPage": {{
            "@type": "WebPage",
            "@id": "{}"
        }}
    }}
    </script>

    <link rel="stylesheet" href="/style.css">

    <style>
        body {{ padding: 20px; }}
        .redirect-notice {{ 
            background: #e7f3ff; 
            border: 1px solid #b3d9ff; 
            padding: 15px; 
            margin: 20px 0; 
            border-radius: 4px; 
        }}
    </style>
</head>
<body>
    <article>
        <h1>{}</h1>
        <div class="post-meta">
            <span class="date">{}</span>
            <span class="author">by Hex</span>
        </div>
        <div class="tags">{}</div>
        
        <div class="redirect-notice">
            <strong>Note:</strong> This is a static version for search engines. 
            <a href="/blog/{}">View the interactive version</a> for the full experience.
        </div>
        
        <div class="content">
            {}
        </div>
    </article>
    
    <script>
        // Redirect browsers to the WASM app, but let crawlers see the content
        if (window.navigator && window.navigator.userAgent && 
            !window.navigator.userAgent.includes('bot') && 
            !window.navigator.userAgent.includes('crawler') &&
            !window.navigator.userAgent.includes('spider') &&
            !window.navigator.userAgent.includes('crawl')) {{
            // Small delay to ensure crawlers can see the content
            setTimeout(function() {{
                window.location.replace('/blog/{}');
            }}, 100);
        }}
    </script>
</body>
</html>"#, 
    html_escape(title), html_escape(summary), tags.join(", "), 
    html_escape(title), html_escape(summary), canonical_url,
    html_escape(title), html_escape(summary), canonical_url,
    html_escape(title), html_escape(summary), base_url, date, canonical_url, canonical_url,
    html_escape(title), date, tags_html, slug, body, slug);

    Ok(html)
}

fn generate_sitemap(post_data: &[PostData], base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut entries = Vec::new();
    
    // Add main pages
    entries.push(format!(r#"    <url>
        <loc>{}</loc>
        <changefreq>monthly</changefreq>
        <priority>1.0</priority>
    </url>"#, base_url));
    
    entries.push(format!(r#"    <url>
        <loc>{}/blog/index.html</loc>
        <changefreq>weekly</changefreq>
        <priority>0.9</priority>
    </url>"#, base_url));
    
    // Add blog posts
    for post in post_data {
        let slug = match &post.values["slug"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let date = match &post.values["date"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        entries.push(format!(r#"    <url>
        <loc>{}/blog/{}.html</loc>
        <lastmod>{}</lastmod>
        <changefreq>monthly</changefreq>
        <priority>0.7</priority>
    </url>"#, base_url, slug, date));
    }
    
    let sitemap = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#, entries.join("\n"));
    
    Ok(sitemap)
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn generate_blog_index_html(post_data: &[PostData], base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut posts_html = String::new();
    
    for post in post_data {
        let slug = match &post.values["slug"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let title = match &post.values["title"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let date = match &post.values["date"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let summary = match &post.values["summary"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let tags = match &post.values["tags"] {
            PostValue::StrSlice(tags) => tags,
            _ => continue,
        };

        let tags_html = tags.iter()
            .map(|tag| format!("<span class=\"tag\">{}</span>", html_escape(tag)))
            .collect::<Vec<_>>()
            .join("");

        posts_html.push_str(&format!(r#"
        <article class="card">
            <h2><a href="/blog/{}">{}</a></h2>
            <p class="date">{}</p>
            <p class="summary">{}</p>
            <div class="tags">{}</div>
        </article>"#, slug, html_escape(title), date, html_escape(summary), tags_html));
    }

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Blog - Hex</title>
    <meta name="description" content="Blog posts by Hex, covering Rust, emerging web standard like WebAssembly, cybersecurity and other technology topics.">
    <meta name="author" content="Vladyslav 'Hex' Yamkovyi">
    <meta name="keywords" content="blog, rust, webassembly, cybersecurity, technology, hex">
    
    <!-- Open Graph -->
    <meta property="og:title" content="Blog - Hex">
    <meta property="og:description" content="Blog posts by Hex, covering Rust, emerging web standard like WebAssembly, cybersecurity and other technology topics.">
    <meta property="og:type" content="website">
    <meta property="og:url" content="{}/blog">
    <meta property="og:site_name" content="Hex's Blog">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="Blog - Hex">
    <meta name="twitter:description" content="Blog posts by Hex, covering Rust, emerging web standard like WebAssembly, cybersecurity and other technology topics.">
    
    <!-- Canonical URL -->
    <link rel="canonical" href="{}/blog">
    
    <link rel="stylesheet" href="/style.css">
    
    <style>
        body {{ padding: 20px; }}
        .redirect-notice {{ 
            background: #e7f3ff; 
            border: 1px solid #b3d9ff; 
            padding: 15px; 
            margin: 20px 0; 
            border-radius: 4px; 
        }}
        .card {{ 
            margin: 20px 0; 
            padding: 20px; 
            border: 1px solid #ddd; 
            border-radius: 8px; 
        }}
        .tags {{ margin-top: 10px; }}
        .tag {{ 
            background: #f0f0f0; 
            padding: 2px 8px; 
            border-radius: 4px; 
            margin-right: 5px; 
            font-size: 0.8em; 
        }}
    </style>
</head>
<body>
    <main>
        <div class="blog-header">
            <h1>Blog</h1>
        </div>
        
        <div class="redirect-notice">
            <strong>Note:</strong> This is a static version for search engines. 
            <a href="/blog">View the interactive version</a> for the full experience.
        </div>
        
        {}
    </main>
    
    <script>
        // Redirect browsers to the WASM app, but let crawlers see the content
        if (window.navigator && window.navigator.userAgent && 
            !window.navigator.userAgent.includes('bot') && 
            !window.navigator.userAgent.includes('crawler') &&
            !window.navigator.userAgent.includes('spider') &&
            !window.navigator.userAgent.includes('crawl')) {{
            // Small delay to ensure crawlers can see the content
            setTimeout(function() {{
                window.location.replace('/blog');
            }}, 100);
        }}
    </script>
</body>
</html>"#, base_url, base_url, posts_html);

    Ok(html)
}

fn generate_index_html(about_data: &PostData, base_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let title = match &about_data.values["title"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid title".into()),
    };
    
    let description = match &about_data.values["description"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid description".into()),
    };
    
    let keywords = match &about_data.values["keywords"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid keywords".into()),
    };
    
    let author = match &about_data.values["author"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid author".into()),
    };
    
    let og_title = match &about_data.values["og_title"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid og_title".into()),
    };
    
    let og_description = match &about_data.values["og_description"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid og_description".into()),
    };
    
    let og_type = match &about_data.values["og_type"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid og_type".into()),
    };
    
    let og_url = match &about_data.values["og_url"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid og_url".into()),
    };
    
    let og_site_name = match &about_data.values["og_site_name"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid og_site_name".into()),
    };
    
    let twitter_card = match &about_data.values["twitter_card"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid twitter_card".into()),
    };
    
    let twitter_title = match &about_data.values["twitter_title"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid twitter_title".into()),
    };
    
    let twitter_description = match &about_data.values["twitter_description"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid twitter_description".into()),
    };
    
    let canonical_url = match &about_data.values["canonical_url"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid canonical_url".into()),
    };
    
    let body = match &about_data.values["body"] {
        PostValue::Str(s) => s,
        _ => return Err("Invalid body".into()),
    };

    // Generate the blog section and footer HTML
    let blog_section = r#"
    <section>
        <h2>Blog</h2>
        <p>I write about technology, security, and engineering philosophy. <a href="/blog">Read my blog posts</a> covering topics from WebAssembly to cybersecurity fundamentals.</p>
    </section>
    
    <footer>
        <p><a href="https://github.com/0x484558">GitHub</a> | <a href="/blog">Blog</a> | <a href="/about">About</a></p>
    </footer>"#;

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
    <meta name="description" content="{}" />
    <meta name="keywords" content="{}" />
    <meta name="author" content="{}" />
    
    <!-- Open Graph -->
    <meta property="og:title" content="{}" />
    <meta property="og:description" content="{}" />
    <meta property="og:type" content="{}" />
    <meta property="og:url" content="{}" />
    <meta property="og:site_name" content="{}" />
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="{}" />
    <meta name="twitter:title" content="{}" />
    <meta name="twitter:description" content="{}" />
    
    <!-- Sitemap -->
    <link rel="sitemap" type="application/xml" title="Sitemap" href="/sitemap.xml" />
    
    <!-- Canonical URL -->
    <link rel="canonical" href="{}" />
    
    <!-- Structured Data -->
    <script type="application/ld+json">
    {{
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "Vladyslav Yamkovyi",
        "alternateName": "Hex",
        "jobTitle": "Chief Technology Officer",
        "worksFor": {{
            "@type": "Organization",
            "name": "aleph0 s.r.o."
        }},
        "url": "{}",
        "sameAs": [
            "https://github.com/0x484558",
            "https://www.linkedin.com/in/0x484558"
        ],
        "knowsAbout": [
            "Rust Programming",
            "WebAssembly",
            "Cybersecurity",
            "Systems Architecture",
            "AI Infrastructure"
        ]
    }}
    </script>
    
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.violet.min.css"/>
    <link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs />
    <link data-trunk rel="css" href="style.css"/>
    <link data-trunk rel="copy-file" href="404.html"/>
    <link data-trunk rel="copy-file" href="target/sitemap.xml"/>
    <link data-trunk rel="copy-file" href="target/robots.txt"/>
    <link data-trunk rel="copy-dir" href="target/blog" data-target-path="blog"/>
</head>
<body>
    <noscript>
    {}
    {}
    </noscript>
    <script>
    </script>
</body>
</html>"#, 
    html_escape(title),
    html_escape(description), 
    html_escape(keywords),
    html_escape(author),
    html_escape(og_title),
    html_escape(og_description),
    html_escape(og_type),
    html_escape(og_url),
    html_escape(og_site_name),
    html_escape(twitter_card),
    html_escape(twitter_title),
    html_escape(twitter_description),
    html_escape(canonical_url),
    base_url,
    body,
    blog_section);

    Ok(html)
}

fn process_about_file() -> Result<PostData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("about.md")?;
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&content);

    let frontmatter: Frontmatter = result
        .data
        .as_ref()
        .ok_or("No frontmatter found in about.md")?
        .deserialize()?;

    let markdown_content = result.content;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(&markdown_content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(PostData::new()
        .set_str("slug", "about")
        .set_str("title", frontmatter.title)
        .set_str("date", frontmatter.date)
        .set_str("summary", frontmatter.summary.unwrap_or_default())
        .set_str("body", html_output)
        .set_str_slice("tags", frontmatter.tags)
        .set_str("raw_content", markdown_content)
        .set_str("description", frontmatter.description.unwrap_or_default())
        .set_str("keywords", frontmatter.keywords.unwrap_or_default())
        .set_str("author", frontmatter.author.unwrap_or_default())
        .set_str("og_title", frontmatter.og_title.unwrap_or_default())
        .set_str("og_description", frontmatter.og_description.unwrap_or_default())
        .set_str("og_type", frontmatter.og_type.unwrap_or_default())
        .set_str("og_url", frontmatter.og_url.unwrap_or_default())
        .set_str("og_site_name", frontmatter.og_site_name.unwrap_or_default())
        .set_str("twitter_card", frontmatter.twitter_card.unwrap_or_default())
        .set_str("twitter_title", frontmatter.twitter_title.unwrap_or_default())
        .set_str("twitter_description", frontmatter.twitter_description.unwrap_or_default())
        .set_str("canonical_url", frontmatter.canonical_url.unwrap_or_default()))
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=about.md");

    // Define the Post structure declaratively
    let post_struct = StructDef::new("Post")
        .add_field(
            FieldDef::new("slug", FieldType::StaticStr)
                .with_doc("URL-friendly identifier for the post"),
        )
        .add_field(
            FieldDef::new("title", FieldType::StaticStr)
                .with_doc("Human-readable title of the post"),
        )
        .add_field(
            FieldDef::new("date", FieldType::StaticStr)
                .with_doc("Publication date in YYYY-MM-DD format"),
        )
        .add_field(
            FieldDef::new("summary", FieldType::StaticStr)
                .with_doc("Brief description of the post content"),
        )
        .add_field(
            FieldDef::new("body", FieldType::StaticStr)
                .with_doc("Rendered HTML content of the post"),
        )
        .add_field(
            FieldDef::new("tags", FieldType::StaticStrSlice)
                .with_doc("Topic tags for categorization and search"),
        )
        .add_field(
            FieldDef::new("raw_content", FieldType::StaticStr)
                .with_doc("Original markdown content for full-text search"),
        );

    // Define the About structure (using same fields as Post for compatibility)
    let about_struct = StructDef::new("About")
        .add_field(
            FieldDef::new("slug", FieldType::StaticStr)
                .with_doc("URL-friendly identifier"),
        )
        .add_field(
            FieldDef::new("title", FieldType::StaticStr)
                .with_doc("Page title"),
        )
        .add_field(
            FieldDef::new("date", FieldType::StaticStr)
                .with_doc("Publication date"),
        )
        .add_field(
            FieldDef::new("summary", FieldType::StaticStr)
                .with_doc("Page description for SEO"),
        )
        .add_field(
            FieldDef::new("body", FieldType::StaticStr)
                .with_doc("Rendered HTML content"),
        )
        .add_field(
            FieldDef::new("tags", FieldType::StaticStrSlice)
                .with_doc("Topic tags"),
        )
        .add_field(
            FieldDef::new("raw_content", FieldType::StaticStr)
                .with_doc("Original markdown content"),
        );

    // Process posts
    let posts_dir = Path::new("posts");
    let mut post_data = Vec::new();

    for entry in fs::read_dir(posts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "md" {
            match process_markdown_file(&path) {
                Ok(data) => post_data.push(data),
                Err(e) => {
                    eprintln!("Warning: Failed to process {}: {}", path.display(), e);
                    continue;
                }
            }
        }
    }

    // Process about page (which is also the index page)
    let about_data = process_about_file()?;

    // Generate code
    let codegen = CodeGen::new()
        .add_struct(post_struct)
        .add_struct(about_struct)
        .add_static_array("POSTS", "Post", post_data.clone())
        .add_static_array("ABOUT", "About", vec![about_data.clone()]);

    // Write Rust output
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("posts.rs");
    let mut posts_file = File::create(&dest_path)?;
    codegen.write_to_file(&mut posts_file)?;

    // Generate SEO-friendly HTML files and assets
    let base_url = "https://0x484558.dev";
    
    // Create target directory structure for web assets
    fs::create_dir_all("target/blog")?;
    
    // Generate HTML files for each post
    for post in &post_data {
        let slug = match &post.values["slug"] {
            PostValue::Str(s) => s,
            _ => continue,
        };
        
        let html_content = generate_html_file(post, base_url)?;
        let html_path = format!("target/blog/{}.html", slug);
        fs::write(&html_path, html_content)?;
        println!("Generated: {}", html_path);
    }
    
    // Generate blog index page
    let blog_index_content = generate_blog_index_html(&post_data, base_url)?;
    fs::write("target/blog/index.html", blog_index_content)?;
    println!("Generated: target/blog/index.html");
    
    // Generate sitemap
    let sitemap_content = generate_sitemap(&post_data, base_url)?;
    fs::write("target/sitemap.xml", sitemap_content)?;
    println!("Generated: target/sitemap.xml");
    
    // Generate robots.txt
    let robots_content = format!(r#"User-agent: *
Allow: /
Sitemap: {}/sitemap.xml
Allow: /blog/*.html
Allow: /blog
Disallow: /dist/
Disallow: /target/
Disallow: /*.wasm
Disallow: /*.js.map"#, base_url);
    
    fs::write("target/robots.txt", robots_content)?;
    println!("Generated: target/robots.txt");
    
    // Generate index.html with trunk directives and about content
    let index_content = generate_index_html(&about_data, base_url)?;
    fs::write("index.html", index_content)?;
    println!("Generated: index.html");
    
    Ok(())
}
