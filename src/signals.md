# Signals

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