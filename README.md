# fltk-anchor

An anchoring mechanism for fltk-rs widgets, useful when resizing the parent to override FLTK's default resizing defaults.

## Usage
```toml
[dependencies]
fltk = 1.1
fltk-anchor = "0.1"
```

## Example

```rust
use fltk::{prelude::*, *};
use fltk_anchor::{Anchor, Anchored};

const PADDING: i32 = 8;

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    button::Button::new(PADDING, PADDING, 80, 40, "Click").with_anchor(Anchor::Left | Anchor::Top);

    input::MultilineInput::new(
        PADDING,
        PADDING * 2 + 40,
        400 - PADDING * 2,
        300 - 40 - PADDING * 3,
        "",
    )
    .with_anchor(Anchor::Left | Anchor::Right | Anchor::Top | Anchor::Bottom);

    win.end();

    win.make_resizable(true);
    win.show();

    a.run().unwrap();
}
```
This indicates to fltk that when resizing, the button has a fixed size and position, while the input fills the remaining part of the window.
