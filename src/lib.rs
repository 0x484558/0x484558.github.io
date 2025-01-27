use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::components::Header;
use crate::pages::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Header />
        <Router>
            <Routes fallback=|| view! { <main><h1>"404 Not Found :("</h1></main> }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
