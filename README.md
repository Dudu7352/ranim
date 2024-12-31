# ranim

**`ranim`** is a small, fast, and simple tool for displaying GIFs directly in your terminal.

## Features

- **Display GIFs in terminal**: Showcase your favorite animations right in the terminal.
- **Customizable dimensions**: Set custom widths, fit the animation to your terminal size (be default tool fills the entirety of the screen).
- **Infinite looping**: Enjoy GIFs endlessly with loop support.
- **Centered display**: Optionally center animations for a cleaner look.

## Installation

> Prerequisites:
> Working stable or nightly rust toolchain

Clone the repository and build `ranim` from source:

```bash
git clone https://github.com/Dudu7352/ranim.git
cd ranim
cargo build --release
cp target/release/ranim ~/.local/bin # this is an example of where to put the binary
```
