# Signals

## Single handler for multiple signals

- https://github.com/fish-shell/fish-shell/blob/545a23734efedd70852c4ab3e0054023687bd57e/src/signal.rs#L162-L192
  > The single signal handler. By centralizing signal handling we ensure that we can never install the "wrong" signal handler
- https://github.com/daanx/isocline/blob/c9310ae58941559d761fe5d2dd2713d245f18da6/src/tty.c#L608-L625
- https://github.com/murarth/mortal/blob/e87c685af2f424be7e4f039baf36f8327d389f71/src/unix/terminal.rs#L390-L406
- https://github.com/dankamongmen/notcurses/blob/50f18543e42f9fd35aed456808ca5adbc15ff5d7/src/lib/unixsig.c#L213-L214

## Termios

### Default mode

```
input_flags: InputFlags(
    BRKINT | ICRNL | IXON | IXANY | IMAXBEL | IUTF8,
), local_flags: LocalFlags(
    ECHOKE | ECHOE | ECHOK | ECHO | ECHOCTL | ISIG | ICANON | IEXTEN | PENDIN,
)
```

### Raw mode (as set by rustyline)

```
input_flags: InputFlags(
    IXANY | IMAXBEL | IUTF8,
), local_flags: LocalFlags(
    ECHOKE | ECHOE | ECHOK | ECHOCTL | PENDIN,
)
```

- `BRKINT` flag removed
> When neither IGNBRK nor BRKINT are set, a BREAK reads as a null byte ('\0')

- `ISIG` flag removed => disable signals

## Alternatives

- isocline
- linefeed: ...
- jline3: `SignalHandler`
