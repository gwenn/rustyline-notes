# Input

## Unix

| Library   | SS3 | Key Seq. timeout | Esc Esc Seq. | Esc [ [ |   |   |   |   |   |
|-----------|-----|------------------|--------------|---------|---|---|---|---|---|
| console   | ko  | ko               |              |   ko    |   |   |   |   |   |
| crossterm | ok  | ko               |              |   ok    |   |   |   |   |   |
| termina   | ok  | ko               |              |   ok    |   |   |   |   |   |
| termwiz   | ok  | ko               |              |   ko    |   |   |   |   |   |


- SS3: F1 on Mac is `EscOP`
- Key Seq. timeout: when using Esc key as Meta key
- Esc [ [: F1 on linux console is `Esc[[A`

### console

<https://github.com/console-rs/console/blob/main/examples/keyboard.rs>
```sh
cargo run --example keyboard
```


### crostterm

<https://github.com/crossterm-rs/crossterm/blob/master/examples/key-display.rs>
```sh
cargo run --example key-display
```


### termina

<https://github.com/helix-editor/termina/blob/master/examples/event-read.rs>
```sh
cargo run --example event-read
```

### termwiz

<https://github.com/wezterm/wezterm/blob/main/termwiz/examples/key_tester.rs>
```sh
cargo run --example key_tester
```
