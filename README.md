# Garnet

Garnet is an experimental UI runtime for describing application interfaces with a small Ruby-like DSL.

The current prototype reads a `.rb` file, builds a UI tree, and renders it with `egui`/`eframe`. It is not a Ruby interpreter yet. Instead, Garnet is exploring what it feels like when interface structure is written in a Ruby-shaped language and then rendered by a Rust runtime.

## What Is Garnet?

Garnet is a Rust application made of a few small layers:

- `garnet-source` loads source files and local assets.
- `garnet-runtime` parses Garnet's Ruby-like DSL into a UI tree.
- `garnet-ui` defines that UI tree.
- `garnet-renderer` renders the UI tree with `egui`.
- `garnet-app` wires everything together as a desktop app.

The core idea is simple:

```ruby
page do
    column do
        image "examples/assets/logo.png"

        text "Garnet"

        row do
            button "Open"
            button "Exit"
        end
    end
end
```

This becomes a tree of `Page`, `Column`, `Image`, `Text`, `Row`, and `Button` nodes, then the renderer draws it.

## Why Ruby Instead Of HTML?

HTML is excellent for documents and the web, but Garnet is exploring a different shape: UI as a small, expressive program.

Ruby gives Garnet a few useful qualities:

- **Readable structure**: `page do ... end`, `column do ... end`, and `row do ... end` make nesting explicit without angle brackets.
- **A language-shaped UI surface**: the DSL can grow toward logic, composition, and reusable components without becoming a template language first.
- **A path toward RWP**: Garnet is being shaped with the idea that sources may eventually come from files, HTTP, RWP, or caches through the same loading boundary.
- **Less document bias**: Garnet is not trying to recreate HTML. It starts from application UI primitives and lets the runtime decide how they render.

This is still early. The point is not that Ruby is universally better than HTML. The point is that Ruby gives Garnet a warm, programmable syntax for describing structured interfaces.

## What Works Today?

Garnet can currently parse and render:

- `page do ... end`
- `column do ... end`
- `row do ... end`
- `text "..."`
- `button "..."`
- `image "..."`

Rows and columns can be nested freely. Buttons are display-only for now; clicking them does nothing. Images are loaded from local PNG/JPEG files.

The runtime also reports errors for unsupported syntax, unexpected `end`, unclosed blocks, and components used outside a block.

## Example

```ruby
page do
    column do
        text "Hello Garnet"

        row do
            button "Open"
            button "Save"
            button "Exit"
        end

        text "Version 0.1"
    end
end
```

Expected shape:

```text
Hello Garnet

[ Open ] [ Save ] [ Exit ]

Version 0.1
```

## Run

```sh
cargo run -p garnet-app
```

The app currently loads `examples/sample.rb`.

## Current Limits

Garnet does not yet support:

- click events
- state management
- component definitions
- Ruby execution
- HTTP images
- SVG/GIF/WebP
- image sizing or fit controls
- styling
- layout constraints beyond simple row/column rendering

Those are future steps. Right now the project is focused on proving the core loop: Ruby-like source, Rust runtime, UI tree, native rendering.
