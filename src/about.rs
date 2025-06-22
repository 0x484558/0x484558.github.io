use leptos::prelude::*;
use leptos_meta::{Title, Meta};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About - Hex" />
        <Meta name="description" content="0x484558 - Blog by Hex; Rust development, security and emerging web standards." />
        <main class="about">
            <h1>"Hi. Call me Hex."</h1>
            <p class="tagline">"Vladyslav \"Hex\" Yamkovyi "<a href="mailto:hex@aleph0.ai">"<hex@aleph0.ai>"</a><br />
            " - Software Developer, Security Engineer & Systems Architect"</p>

            <section>
                <p>"I specialize in building secure and performant systems at the intersection of systems programming and infrastructure automation, grounded in due diligence and methodical analysis. My work focuses on creating robust software architectures that can withstand both operational complexity and adversarial conditions."</p>
                <p>"Engineering is a philosophy of life. It is a way of thinking that extends beyond code; it is how one approaches problems, makes decisions, and understands the world. Good engineering requires understanding systems at multiple levels of abstraction simultaneously - and being an eternal learner."</p>
                <p>"I write - and when I do, I explore the philosophical implications of different ways of technology in a large picture, while keeping real-world implementation challenges in mind. Each system tells a story about the trade-offs its designers were willing to accept, and understanding these narratives is crucial to building better technology."</p>
            </section>
        </main>
    }
}
