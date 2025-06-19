use leptos::prelude::*;
use leptos_router::{components::*, params::Params, hooks::use_params};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Params, PartialEq, Clone, Debug)]
struct PostParams {
    slug: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub body: &'static str,
}

fn get_posts() -> &'static [Post] {
    POSTS
}

#[component]
pub fn Blog() -> impl IntoView {
    let posts = get_posts();
    view! {
    <main>
        <h1>"Blog"</h1>
        <For
            each=move || posts.iter().cloned()
            key=|post| post.slug
            children=|post| view! {
                <article class="card">
                    <h2><A href=post.slug>{post.title}</A></h2>
                    <p>{post.date}</p>
                </article>
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
    <main>
        {move || match post() {
            Some(p) => view! {
                <article>
                    <h1>{p.title}</h1>
                    <p>{p.date}</p>
                    <div inner_html=p.body></div>
                </article>
            },
            None => {
                let title: &str = "404";
                let date: &str = "Post not found.";
                let body: &str = "";
                view! { 
                    <article>
                        <h1>{title}</h1>
                        <p>{date}</p>
                        <div inner_html=body></div>
                    </article>
                }
            },
        }}
    </main>
    }
}
