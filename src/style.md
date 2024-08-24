# Style

Currently, rustyline `Highlighter` uses ANSI escape sequence directly.
But ...

## Impl

- <https://docs.rs/crossterm/latest/crossterm/style/index.html>
- <https://docs.rs/lineeditor/0.1.0/lineeditor/style/struct.Style.html>
- <https://docs.rs/liso/latest/liso/struct.Style.html>
- <https://docs.rs/mortal/latest/mortal/terminal/struct.Style.html>
- <https://docs.rs/nu-ansi-term/latest/nu_ansi_term/struct.Style.html>
- <https://docs.rs/ratatui/latest/ratatui/style/index.html>
- <https://docs.rs/termion/latest/termion/style/index.html>
- <https://docs.rs/termwiz/latest/termwiz/cell/struct.CellAttributes.html>
- <https://docs.rs/tty-interface/latest/tty_interface/struct.Style.html>

## ANSI Parser

- <https://github.com/qwandor/anes-rs>
- <https://github.com/rust-cli/anstyle/tree/main/crates/anstyle-parse> (~ vte fork)
- <https://github.com/colored-rs/cansi>
- <https://gitlab.redox-os.org/redox-os/ransid>

## Misc

- <https://github.com/zhiburt/ansi-str>
- <https://gitlab.com/zhiburt/ansitok>
- <https://github.com/dandavison/vte-ansi-tools/>

## Blocks

Ideally, `Highlighter` should not apply style directly on input text.
But should return `Styled blocks` so that `rustyline` can easily split some blocks when needed (for scrolling, continuation prompt, ...).

`ansi-str` crate (see `get_blocks`) can be used to keep compatibility with existing implementation.
But its `Style` struct is immutable / unusable.

`anstyle` crate can be used instead as a default / optional implementation.