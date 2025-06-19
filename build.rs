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
}

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let posts_dir = Path::new("posts");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts.rs");
    let mut posts_file = File::create(&dest_path).unwrap();

    let mut posts = Vec::new();

    for entry in fs::read_dir(posts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "md" {
            let slug = path.file_stem().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path).unwrap();

            let matter = Matter::<YAML>::new();
            let result = matter.parse(&content);

            let frontmatter: Frontmatter = result.data.as_ref().unwrap().deserialize().unwrap();
            let markdown_content = result.content;

            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&markdown_content, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            posts.push(format!(
                r#"
                Post {{
                    slug: "{}",
                    title: "{}",
                    date: "{}",
                    body: "{}",
                }}
                "#,
                slug,
                frontmatter.title,
                frontmatter.date,
                html_output.escape_default().to_string()
            ));
        }
    }

    write!(
        posts_file,
        r#"static POSTS: &[Post] = &[
{}
];"#,
        posts.join(r#",
"#)
    )
    .unwrap();
}
