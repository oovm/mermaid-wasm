Mermaid.js for Yew
==================

Render diagram with Mermaid.js in Yew!

- The online preview: https://galaster.github.io/yew-mermaid.js

## How to use

1. No CDN needed

2. Easily added using `<Mermaid/>`

```rust
use yew_mermaid::Mermaid;

html! {
    <Mermaid code=&self.input theme="neutral"/>
}
```

## Todo

- [ ] Fix the problem that the entire wasm vm crashes when rendering errors
