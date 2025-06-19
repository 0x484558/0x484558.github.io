use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod about;
pub mod blog;

use crate::about::About;
use crate::blog::{Blog, BlogPost};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
    <Header />
    <Router>
        <Routes fallback=|| view! { <main><h1>"404 Not Found :("</h1></main> }>
            <Route path=path!("/") view=Home />
            <Route path=path!("/blog") view=Blog />
            <Route path=path!("/blog/:slug") view=BlogPost />
        </Routes>
    </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
    <header>
        <hgroup>
        <h1><a href="/">Hex</a></h1>
        <p>0x484558</p>
        </hgroup>
        <nav>
            <ul>
                <li><a href="/blog">Blog</a></li>
                <li><a href="https://github.com/0x484558">GitHub</a></li>
            </ul>
        </nav>
    </header>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! { <About /> }
}
