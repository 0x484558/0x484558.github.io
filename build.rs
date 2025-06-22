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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=posts");

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

    // Generate code
    let codegen = CodeGen::new()
        .add_struct(post_struct)
        .add_static_array("POSTS", "Post", post_data);

    // Write output
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("posts.rs");
    let mut posts_file = File::create(&dest_path)?;

    codegen.write_to_file(&mut posts_file)?;
    Ok(())
}
