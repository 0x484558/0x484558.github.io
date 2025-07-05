use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::{hooks::use_params, params::Params};
use wasm_bindgen::JsCast;

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Params, PartialEq, Clone, Debug)]
struct PostParams {
    slug: Option<String>,
}

fn get_posts() -> &'static [Post] {
    POSTS
}

#[component]
pub fn Blog() -> impl IntoView {
    let posts = get_posts();
    let (search, set_search) = signal(String::new());

    let filtered_posts = move || {
        let search_term = search.get().to_lowercase();
        if search_term.is_empty() {
            posts.to_vec()
        } else {
            posts
                .iter()
                .filter(|p| {
                    p.title.to_lowercase().contains(&search_term)
                        || p.summary.to_lowercase().contains(&search_term)
                        || p.raw_content.to_lowercase().contains(&search_term)
                        || p.tags
                            .iter()
                            .any(|t| t.to_lowercase().contains(&search_term))
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    };

    view! {
        <Title text="Blog - Hex" />
        <Meta name="description" content="blog posts by Hex, covering Rust, emerging web standard like WebAssembly, cybersecurity and other technology topics." />
        <main>
        <div class="blog-header">
            <h1>"Blog"</h1>
            <input
                type="text"
                placeholder="Search posts..."
                class="search-input"
                on:input=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                            set_search.set(input.value());
                        }
                    }
                }
                prop:value=search
            />
        </div>
        <For
            each=filtered_posts
            key=|post| post.slug.to_string()
            children=|post| {
                view! {
                    <article class="card">
                        <h2><a href=format!("/blog/{}", post.slug)>{post.title}</a></h2>
                        <p class="date">{post.date}</p>
                        {if !post.summary.is_empty() {
                            view! { <p class="summary">{post.summary}</p> }
                        } else {
                            view! { <p class="summary">""</p> }
                        }}
                        <div class="tags">
                            {post.tags.iter().map(|tag| {
                                view! { <span class="tag">{*tag}</span> }
                            }).collect_view()}
                        </div>
                    </article>
                }
            }
        />
        </main>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = move || {
        let slug = params.get().ok()?.slug?;
        get_posts().iter().find(|p| p.slug == slug).cloned()
    };

    view! {
    <Title text=move || post().as_ref().map(|p| format!("{} - Hex", p.title)).unwrap_or("404 - Hex".to_string()) />
    <Meta name="description" content=move || post().as_ref().map(|p| p.summary.to_string()).unwrap_or("Requested blog post not found.".to_string()) />
    <main>
        {move || {
            let post_data = post();
            let title = post_data.as_ref().map(|p| p.title).unwrap_or("404");
            let date = post_data.as_ref().map(|p| p.date).unwrap_or("Post not found.");
            let tags = post_data.as_ref().map(|p| p.tags).unwrap_or(&[]);
            let body = post_data.as_ref().map(|p| p.body).unwrap_or("");

            view! {
                <article>
                <h1>{title}</h1>
                <p>{date}</p>
                <div class="tags">
                    {tags.iter().map(|tag| {
                        view! { <span class="tag">{*tag}</span> }
                    }).collect_view()}
                </div>
                <div inner_html=body></div>
                </article>
            }
        }}
    </main>
    }
}
