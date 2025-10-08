# [0x484558.dev](https://0x484558.dev)

This is a static website built with [Zola](https://www.getzola.org/), a fast static site generator written in Rust. The site features a blog and portfolio content, styled with [Pico CSS](https://picocss.com/) and semantic HTML5.

## Writing

```sh
# Build the site for production
zola build
# Serve the site locally with live reload
zola serve
# Build and check for broken links
zola check
```

- Blog posts go in `content/blog/`
- Use front matter to set metadata (title, date, tags, etc.)
- The site supports tags taxonomy for organizing posts
- RSS feed is automatically generated at `/rss.xml`

## License

Copyright (c) 2025 [0x484558](https://github.com/0x484558).

This work is provided under a dual-license arrangement (MIT & CC-BY-4.0).

1. The software components, including but not limited to all source code files, build scripts, and configuration files, are licensed under the MIT License. A copy of this license is available in the [LICENSE](LICENSE) file.

2. All other materials, including but not limited to documentation, text content, images, multimedia assets, and any non-software components, are licensed under the Creative Commons Attribution 4.0 International License (CC-BY-4.0). The full text of this license is available at [https://creativecommons.org/licenses/by/4.0/legalcode](https://creativecommons.org/licenses/by/4.0/legalcode).
