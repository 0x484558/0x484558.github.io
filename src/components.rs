use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <hgroup>
            <h1><a href="/">0x484558</a></h1>
            <p>"a.k.a. Hex"</p>
            </hgroup>
            <nav>
                <ul>
                    <li><a href="https://github.com/0x484558">GitHub</a></li>
                </ul>
            </nav>
        </header>
    }
}
