use leptos::prelude::*;
use leptos_meta::{Meta, Title};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

fn get_about() -> &'static About {
    &ABOUT[0]
}

#[component]
pub fn About() -> impl IntoView {
    let about = get_about();
    
    view! {
        <Title text=format!("{} - Hex", about.title) />
        <Meta name="description" content=about.summary />
        <main class="resume" inner_html=about.body>
        </main>
    }
}
