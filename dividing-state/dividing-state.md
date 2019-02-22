---
---

# Dividing State into Components

Cool! Magic! Awesome! This code is from [src/main.rs:5].
This link should be broken [Cargo.toml]. This link should be to download the
[Asset Archive][assets.zip] file.

```rust
mod window;

use window::Window;

fn main() {
    let win = Window::new("Hello, window!");
    println!("{:?}", win);
}
```

Running `cargo run` produces:

```
Compiling game-tutorial v0.1.0 (/path/to/game-tutorial)
 Finished dev [unoptimized + debuginfo] target(s) in 6.76s
```

Some JavaScript:

```js
function setupNavbar() {
  const tocToggle = document.querySelector('.tour-toc-toggle');
  // The toc-toggle is disabled for content-only pages
  if (!tocToggle) { return; }

  const toc = document.querySelector('.tour-toc');
  const stepContent = document.querySelector('.tour-step-content');

  tocToggle.addEventListener('click', (e) => {
    tocToggle.classList.toggle('active');
    toc.classList.toggle('hidden');
    stepContent.classList.toggle('hidden');
  });
}
```

Some indented code:

    print "Hello, world!"
    input = raw_input()
