# [0x484558.dev](https://0x484558.dev)

This is a Client-Side Rendered [Leptos](https://leptos.dev/) application, built with [Rust](https://www.rust-lang.org/) compiled to [WebAssembly](https://webassembly.org/) and [semantic HTML5](https://web.dev/learn/html/semantic-html/) styled with [Pico CSS](https://picocss.com/).

## Building

```sh
# Add WASM target to Rust tooling
rustup target add wasm32-unknown-unknown
# Install Trunk bundler
cargo install trunk
# Use Trunk to build the web app.
trunk build --release
```

## License

Copyright (c) 2025 [0x484558](https://github.com/0x484558) & [Tibor "Jack" Sloboda](https://github.com/slobodaapl).

This work is provided under a dual-license arrangement (MIT & CC-BY-4.0).

1. The software components, including but not limited to all source code files, build scripts, and configuration files, are licensed under the MIT License. A copy of this license is available in the [LICENSE](LICENSE) file.

2. All other materials, including but not limited to documentation, text content, images, multimedia assets, and any non-software components, are licensed under the Creative Commons Attribution 4.0 International License (CC-BY-4.0). The full text of this license is available at [https://creativecommons.org/licenses/by/4.0/legalcode](https://creativecommons.org/licenses/by/4.0/legalcode).
