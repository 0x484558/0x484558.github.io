---
title: The War for the Web
date: 2025-06-20
---

You're reading something unusual right now. These words weren't served from a database or rendered by a server. They weren't even loaded from a static file. Instead, they emerged from WebAssembly bytecode compiled Rust that carries its own content - a **self-contained web application**. Every blog article, including this one, is transformed from Markdown to Rust view code at compile time and embedded directly into the WASM binary you downloaded when you visited this site.

Traditional blogs follow one of these patterns:
- **Dynamic sites** query databases and render HTML on each request;
- **Static site generators** pre-build HTML files that get served directly;
- **Client-side apps** fetch content via APIs and render it in the browser.

This blog? It's none of those, yet somehow all of them at once.

## The Web's Battleground

To understand why this blog takes such an unconventional approach, we need to examine the larger conflict reshaping web development. The modern web has become a battlefield where three competing philosophies vie for dominance:

- **The Empire of JavaScript**: The stagnant regime backed by "RESTful" JSON APIs. React, Vue, Angular. Thick clients that treat the browser as an application runtime and manipulate on DOM, shadow or not. An ecosystem of staggering complexity, built on mountains of npm packages, baroque build tools, and a new state management library every Tuesday. It's a world where "RESTful" means shipping megabytes of client-side code to fetch JSON objects, a grotesque parody - the server becomes a data layer, the frontend becomes a complex state machine, and the network becomes a high-latency bus between them. This is what most people call "modern web development" - and this empire, ruled by React, promises application-like experiences but mostly delivers churn, complexity, and abysmal performance.
- **The Hypermedia Reformation**: The righteous rebellion of HTMX. [As Carson Gross argues](https://htmx.org/essays/how-did-rest-come-to-mean-the-opposite-of-rest/), the JavaScript Empire has forgotten what made the web work. HTMX is a return to sanity, a declaration that you can have rich interactivity without drowning in client-side state. This movement represents a direct and necessary assault on the incidental complexity that the Empire normalized - by returning to *delivering documents*.
- **The Compiled Frontier**: The parallel rebellion of WebAssembly. This path appeals to those who believe in performance, type safety, and engineering discipline. WebAssembly isn't just for running games or CAD software in a browser; it offers a complete escape route from the chaos of the JavaScript ecosystem. This approach sees the browser not as a JavaScript runtime, but as a universal, sandboxed compilation target - a means to an end of *delivering code* over the network.

Both HTMX and WebAssembly represent movements of dissent against the same broken status quo. I acknowledge efforts of both equally, but personally salute the Reformation's diagnosis of the problem, which in my view offers a very meaningful path forward that goes beyond supplying tools to low-effort companies for extracting money from venture capital. The HTMX community's celebrated [port of a WASM app](https://htmx.org/essays/a-real-world-wasm-to-htmx-port/) wasn't a victory for HTML over compiled code; it was a victory for simplicity over the kind of needless Rube Goldberg complexity the JavaScript Empire taught us was normal.

The point is - *we agree on the enemy*. The question is not "which is less complex," but "what kind of complexity are you willing to accept?" The Empire's complexity is accidental, meaningless, and serves only to feed the churn. The complexity of the rebellions is deliberate and purposeful.

- **HTMX chooses the complexity of the server**. It asks you to manage state, templates, and routing on the backend, in exchange for radical simplicity on the frontend. It is a bet on the robustness of server-side logic and the enduring power of HTML.
- **WebAssembly chooses the complexity of the toolchain**. It asks you to accept compile times, binary sizes and low-level complexity in exchange for potentially escaping JavaScript sandbox and DOM entirely and allows writing sophisticated software in a truly OS-independent and standardized to the bytecode way. It is a bet on performance, portability, and the power of compiled languages delivered by an unsophisticated server-side logic.

Neither is wrong. They are two different fronts in the same war. HTMX is fighting a guerilla war from the hills, using the enemy's own infrastructure against them. WebAssembly is opening a new front across the sea, with new weapons and a new philosophy of combat.

## Subversion with WebAssembly

This blog's architecture embodies the WebAssembly philosophy in its purest form. But to appreciate what makes this approach novel, a brief historical parallel illuminating the context is necessary.

JavaServer Pages (JSP) pioneered the idea of compiling markup into executable code - transforming `.jsp` files into Java servlets that could generate HTML. Each JSP page became a compiled class, with HTML templates converted into print statements and dynamic content woven through Java code. This compilation approach offered performance benefits and type safety, but at the cost of mixing presentation with business logic.

WebAssembly takes this compilation concept and inverts it: instead of server-side compilation producing HTML on every request, we have client-side compilation *containing* view logic that renders once in your browser. Where JSP compiled templates into bytecode that ran on the JVM to produce markup repeatedly, this blog compiles markdown into WebAssembly bytecode that runs in the browser to produce markup on demand. We've returned to treating content as code, but now the compilation target is the browser itself.

When you navigated here, your browser downloaded a WebAssembly module. Inside that binary, alongside the [Leptos framework](https://leptos.dev/) and routing logic, lives the compiled view functions for every blog post. This isn't just a technical curiosity - it's a statement about what single-page applications could be. Yes, this blog is an SPA. But unlike the JavaScript Empire's SPAs that ship megabytes of interpreted code and runtime dependencies, this ships a single optimized binary. The difference isn't in the user experience; it's in the engineering discipline required to create it.

JSP moved templating complexity into compiled server code. The JavaScript Empire moved it into client-side state management. HTMX moves it back to the server but keeps it simple. WebAssembly says: if we're going to have complexity, let's at least have the benefits of a real compiler and type system.

This represents just one of many plausible solutions to the tension between seeing the web as a document platform or an application platform. The industry has spent decades pretending otherwise, layering application logic on top of document semantics until the original intent became unrecognizable. WebAssembly represents the possibility for philosophical reconciliation. The technology declares: "Let's stop pretending. The web is an application platform" - and brings the full power of desktop applications to the web. Consider WebGPU, which brings near-native graphics performance to the web, and the emerging WebNN, which will allow running machine learning and artificial intelligence using on-device resources, in the browser. And yet, paradoxically, this blog uses that application platform to deliver... documents. The circle is complete, using the web's most advanced application delivery mechanism to serve what the web was originally designed for - hypertext.

## The Trade-off

When bots visit this WASM-powered site, they find an empty shell:

```html
<!DOCTYPE html>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="..."/>
<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs />
<title>...</title>
<body></body>
```

Yes, this approach violates "view source" transparency. But here's the thing: so does every React app with `<div id="root">`-ridden DOM. The difference is that WebAssembly is honest about the trade-off. This architecture chooses compilation over interpretation, accepting the trade-offs explicitly rather than pretending that generated `<div class="css-1a2b3c">` soup is semantic HTML. 

For SEO-critical applications, WebAssembly developers can employ server-side rendering (SSR) or static pre-rendering to generate indexable HTML snapshots. Tools like Leptos support SSR out of the box, allowing the same Rust code to render on both server and client. This hybrid approach delivers crawlable content for search engines while maintaining the performance benefits of client-side WebAssembly.

Each architectural choice carries real costs in practice:

**HTMX**: 
- **Wins**: Sub-14KB library, progressive enhancement, SEO-friendly by default, works without JavaScript
- **Costs**: Server state management, websocket complexity for real-time features, careful API design
- **Choose when**: Building content-heavy sites, e-commerce, anything where SEO matters, teams with strong backend expertise

**WebAssembly**:
- **Wins**: Type safety, near-native performance, true offline capability, sophisticated client-side features
- **Costs**: Baseline binary size, compile times, SEO requires pre-rendering, smaller talent pool
- **Choose when**: Building actual applications (not sites), offline-first experiences, computation-heavy features, when you need to escape JavaScript's limitations

But the real question isn't "Is this overengineered?" The question is "What kind of engineering do you want to do?" HTMX asks you to engineer your server. WebAssembly asks you to engineer your build pipeline. The JavaScript Empire asks you to engineer around its decadency.

## Looking Forward

So where does this leave us? In a battle between two honest rebellions and one dishonest empire.

- HTMX is honest. The framework embraces the web's document-oriented nature and extends it.
- WebAssembly is honest. The technology embraces the web's application-runtime nature and extends it.
- The JavaScript Empire is dishonest. This ecosystem pretends to embrace the document model while reducing HTML to a meaningless delivery vehicle for its bloated client-side machinery. The approach pays lip service to web standards while violating their spirit.

But honesty alone doesn't ship products. Here's your practical path forward:

**If you're starting fresh:**
1. **Default to HTMX** for anything content-focused. It's not just simpler; it's more correct for 80% of web projects.
2. **Consider WebAssembly** when you have actual application requirements: offline-first, heavy computation, or when you need guarantees that JavaScript can't provide.
3. **Avoid the Empire** unless you have a specific technical requirement that only its ecosystem can satisfy.

**If you're already in the Empire:**
1. **Stop adding complexity**. No new state management libraries. No new build tool chains. No new abstractions.
2. **Start extracting**. Move pages that don't need client state to server-rendered alternatives. You don't have to rewrite everything.
3. **Measure ruthlessly**. Bundle size, time-to-interactive, memory usage. Let data drive your migration.

**For the adventurous:**
1. **Experiment with hybrid approaches**. HTMX for your content, WebAssembly for your apps.
2. **Contribute to the rebellions**. Both movements need better tooling, documentation, and real-world examples.
3. **Share your war stories**. Every migration from the Empire strengthens the rebellion.

The future of the web won't be won by a single victor. It will be won by developers who choose architectures that match their actual requirements rather than their tribal affiliations. Choose your complexity wisely, engineer deliberately. But whatever you do, stop pretending that `<div>`-ridden HTML is anyhow "semantic".

---

*P.S. If you're reading this, your browser successfully executed thousands of lines of compiled Rust just to show you these words. That's not inefficient or poetic. It's a declaration of independence.*
