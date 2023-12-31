# Nix

### VMIN = 1, VTIME = 0 => blocking read

> After the initial byte of input becomes available, a timer is restarted as each further byte is received. The read() returns when either the lesser of MIN bytes or the number of bytes requested have been read, or when the time between receiving successive bytes exceeds TIME tenths of a second. Since the timer is started only after the initial byte becomes available, at least 1 byte is returned. (A read() can block indefinitely for this case.)

> This case is useful for handling terminal keys that generate escape sequences. For example, on many terminals, the left-arrow key generates the 3-character sequence consisting of Escape followed by OD. These characters are transmitted in quick succession. Applications handling such sequences need to distinguish the pressing of such a key from the situation where the user slowly types each of the characters individually. This can be done by performing a read() with a small interbyte timeout, say 0.2 seconds. Such a technique is used in the command mode of some versions of vi. (Depending on the length of the timeout, in such applications, we may be able to simulate a left-arrow key press by quickly typing the aforementioned 3-character sequence.)

### libc / nix usages

- isatty
- read / write / close
- tcgetattr / tcsetattr
- ioctl TIOCGWINSZ
- sigaction SIGWINCH
- pipe
- kill SIGTSTP
- poll
- select

### Get cursor position

1) write `\x1b[6n` to stdout
2) scan `\x1b[...;...R` from stdin 

Seems unreliable (when others events occur at the same time)