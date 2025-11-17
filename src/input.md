# Input

## Ansi escape sequence parser impls

- Console: (doesn't seem to handle Esc-X as Alt-X (no timeout on single Esc))

<https://github.com/console-rs/console/blob/de2f15a31a8fef0b0e65ef4bdf92cd03c3061dac/src/unix_term.rs#L207-L284>

- Crossterm:

<https://github.com/crossterm-rs/crossterm/blob/08762b3ef4519e7f834453bf91e3fe36f4c63fe7/src/event/sys/unix/parse.rs#L45-L75>

- Fish shell:

<https://github.com/fish-shell/fish-shell/blob/master/src/input_common.rs>

- Termina (helix)

<https://github.com/helix-editor/termina>

- Termion:

<https://github.com/redox-os/termion/blob/f2b8517c3185d8a6384109c7309589aa9ad48b49/src/event.rs#L117>

- Termwiz:

<https://github.com/wez/wezterm/blob/fec90ae04bf448d4b1475ba1d0ba1392846a70d6/termwiz/src/input.rs#L1075-L1082>
(no usage of `vtparse` by default ?)

- Textmode:

<https://github.com/doy/textmode/blob/73248fa5804238e2a267a63c9614cf60538956a8/src/private.rs#L112-L152>

- Rustyline:

<https://github.com/kkawakam/rustyline/blob/435ae555de5876cbdc7d0350fe32c4939d800ee7/src/tty/unix.rs#L260C22-L260C22>

## Fake input

On unix with `TIOCSTI`:
<https://man7.org/linux/man-pages/man2/TIOCSTI.2const.html>

Used by [isocline](https://daanx.github.io/isocline/group__async.html) to stop the loop.

## Issues

- [vte](https://github.com/alacritty/vte/issues/101)
- [vtparse](https://github.com/wez/wezterm/issues/4463)
