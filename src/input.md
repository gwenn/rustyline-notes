# Input

## Ansi escape sequence parser impls

* Crossterm

<https://github.com/crossterm-rs/crossterm/blob/08762b3ef4519e7f834453bf91e3fe36f4c63fe7/src/event/sys/unix/parse.rs#L45-L75>

* Termion:

<https://github.com/redox-os/termion/blob/f2b8517c3185d8a6384109c7309589aa9ad48b49/src/event.rs#L117>

* Termwiz:

<https://github.com/wez/wezterm/blob/fec90ae04bf448d4b1475ba1d0ba1392846a70d6/termwiz/src/input.rs#L1075-L1082>
(no usage of `vtparse` by default ?)

* Rustyline:

<https://github.com/kkawakam/rustyline/blob/435ae555de5876cbdc7d0350fe32c4939d800ee7/src/tty/unix.rs#L260C22-L260C22>

## Issues

* [vte](https://github.com/alacritty/vte/issues/101)
* [vtparse](https://github.com/wez/wezterm/issues/4463)