use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About - Hex" />
        <Meta name="description" content="CV of Vladyslav 'Hex' Yamkovyi - Rust Software Developer, Security Engineer & Systems Architect." />
        <main class="resume">
        <header>
        <h1>"Vladyslav \"Hex\" Yamkovyi"</h1>
        <p class="tagline">"Software Developer, Security Engineer & Systems Architect"</p>
        <p class="header-contact">
            <a href="mailto:hex@aleph0.ai">"hex@aleph0.ai"</a>
            <span class="separator">" | "</span>
            "Bratislava, Slovakia"
        </p>
        </header>

        <section class="intro">
            <p>"I specialize in building secure and performant systems at the intersection of systems programming and infrastructure automation, grounded in due diligence and methodical analysis. My work focuses on creating robust software architectures that can withstand both operational complexity and adversarial conditions."</p>
            <p>"Engineering is a philosophy of life. It is a way of thinking that extends beyond code; it is how one approaches problems, makes decisions, and understands the world. Good engineering requires understanding systems at multiple levels of abstraction simultaneously - and being an eternal learner."</p>
        </section>

        <hr />

        <section class="experience">
        <h2>"Professional Experience"</h2>

        <article class="item">
            <h3>"Backend Engineer, AI R&D"</h3>
            <p class="meta">"NetFire LLC | 2024 – present"</p>
            <p>"Working under the supervision of Ing. Tibor Sloboda, VP of AI Strategy and Doctoral Researcher at Vision and Graphics Group, FIIT STU, I design and implement robust and efficient infrastructure for AI research and development."</p>
            <p>"I bring value by combining past experiences, theoretical computer science concepts and practical challenges posed by emerging domain of Artificial Intelligence operations."</p>
        </article>

        <article class="item">
            <h3>"Rust Developer"</h3>
            <p class="meta">"Elastio Inc. | 2021 – 2022"</p>
            <p>"Development of data plane component of a cloud-native backup solutions with focus on memory safety, reliable concurrent programming, and ransomware protection."</p>
            <p>"My contribution included development of comprehensive virtualization-based testing suite to facilitate support for ARM-based processors, implementing core components for the backup agent and integration with AWS services, and extensive use of advanced Rust construct and higher-ordered typing to enhance system flexibility and reliability."</p>
        </article>

        <article class="item">
            <h3>"Software Engineer"</h3>
            <p class="meta">"Telekart-Prylad LLC, Ukraine | 2019 – 2020"</p>
            <p>"Focused on critical infrastructure systems for military applications, research and development on packet transport optimization in low-bandwidth radio networks, and secure infrastructure operations."</p>
        </article>
        </section>

        <section class="education">
            <h2>"Education"</h2>
            <p class="meta">"Master of Computer Science @ State University of Information and Communication Technology, Ukraine"</p>
            <p>"My thesis \"Investigation of hyperbolic geometry of computer networks\" received \"Excellent\" grade for demonstration of hyperbolic data structure achieving O(log n) spatial complexity for computer network representation."</p>
        </section>

        <hr />

        <section class="skills">
        <article class="item">
            <h3>"Skills"</h3>
            <p><strong>"Programming Languages: Rust, Go, C"</strong>", Java, C#, C++, Python, Bash"</p>
            <p><strong>"Technologies: "</strong>"Kubernetes, Podman, PostgreSQL, Linux ops with systemd, virtualization (KVM/QEMU) and computer networking"</p>
            <p><strong>"Natural Languages: "</strong>"English, Ukrainian, Russian"</p>
        </article>
        <article class="item">
            <h3>"Interests"</h3>
            <p><strong>"Security, Reliability & Cryptography: "</strong>"TLS/PKI Infrastructures, Threat Modeling & Security Analysis, Secure Development in Rust, Zero-Knowledge Proofs, Byzantine Fault Tolerance, Formally Verifiable Systems"</p>
            <p><strong>"Distributed & Local-First Computing: "</strong>"Conflict-free Replicated Datatypes (CRDTs), Decentralized Protocols for Digital Sovereignty, Peer-to-Peer Architectures"</p>
            <p><strong>"AI/ML Technologies: "</strong>"Chaotic Dynamics of Attention, Scalable ML Infrastructure, On-device LLM Inference, AI Security"</p>
        </article>
        </section>

        <hr />

        <section class="philosophy">
            <p>"Through writing, I explore the intersection of philosophy and technology. Every system tells a story about the trade-offs its designers were willing to accept and the historical conditions in which it was conceived. Understanding these narratives is crucial to building better technology - and a better future."</p>
        </section>
        </main>
    }
}
