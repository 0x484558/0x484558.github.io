---
title: The Compiled Blog
date: 2025-06-19
---

You're reading something unusual right now. These words weren't served from a database or rendered by a server. They weren't even loaded from a static file. Instead, they emerged from WebAssembly bytecode compiled Rust that carries its own content - a **"self-contained web application"**. Every blog article, including this one, is transformed from Markdown to HTML at compile time and embedded directly into the WASM binary you downloaded when you visited this site.

Traditional blogs follow one of these patterns:
- **Dynamic sites** query databases and render HTML on each request;
- **Static site generators** pre-build HTML files that get served directly;
- **Client-side apps** fetch content via APIs and render it in the browser.

This blog? It's none of those, yet somehow all of them at once.

When you navigated here, your browser downloaded a single WASM module. Inside that binary, alongside the [Leptos framework](https://leptos.dev/) and routing logic, lives the pre-rendered HTML of every blog post. The Markdown→HTML transformation happened at compile time, not runtime. The content is quite literally part of the application's compiled code.

## Performance through Paradox

There's something philosophically satisfying about this approach. In an era where content and code are typically separated - where CMSs, APIs, and databases create layers of abstraction - this blog collapses that distinction. The content *is* the code. The blog posts aren't data to be fetched; they're constants compiled into the binary.

Counterintuitively, embedding content in WASM can be remarkably efficient:
- **Single download**: One WASM file contains everything
- **No API calls**: Content is already in memory
- **Instant routing**: Navigation happens at the speed of function calls
- **Perfect caching**: The content hash *is* the binary hash

The tradeoff? Every new blog post requires recompilation. But in a world of CI/CD pipelines, is that really a limitation or just another build step?

But here's where it gets interesting from a technical perspective. Most web scrapers and bots operate on a simple principle: fetch HTML, parse it, extract content. But when they visit this site, all they find is:

```html
<!DOCTYPE html>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="..."/>
<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs />
<title>...</title>
<body></body>
```

An empty body. The content only materializes when the WASM module executes. To scrape this blog, a bot would need to:
1. Download the WASM binary
2. Initialize a WebAssembly runtime
3. Execute the compiled Rust code
4. Wait for the virtual DOM to render
5. Then extract the dynamically generated content

It's an unintentional but elegant form of content protection - not through encryption or paywalls, but through the fundamental requirement of computation.

## Looking Forward

This blog is an experiment in collapsing boundaries - between content and code, between static and dynamic, between server and client. It's a rejection of the traditional web's separation of concerns in favor of something more unified, more compiled, more... singular. If you're reading this, it means your browser successfully executed thousands of lines of compiled Rust just to show you these words. That's either incredibly inefficient or surprisingly poetic. I prefer to think it's both.
