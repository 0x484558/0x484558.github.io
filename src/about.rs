use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <main>
        <p>"I am an engineer with a passion for crafting elegant and robust software."</p>
        <h2>"Interests"</h2>
        <ul>
            <li>"🦀 "<a href="https://www.rust-lang.org/">"Rust"</a>" development and systems programming."</li>
            <li>"🛠 DevOps and infrastructure automation."</li>
            <li>"🔒 Security-focused development practices."</li>
        </ul>
        <p>"When I am not immersed in code, you will find me:"</p>
        <ul>
            <li>"🎧 Discovering new music."</li>
            <li>"💭 Engaging in thought-provoking discussions."</li>
            <li>"🌟 Exploring emerging technologies."</li>
            <li>"☕ Enjoying coffee."</li>
        </ul>
        </main>
    }
}
