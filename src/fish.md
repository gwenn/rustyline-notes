# Fish shell

<https://github.com/fish-shell/fish-shell>

## Reader (*)

- src/reader.rs

### Config

`ReaderConfig`
- left_prompt_cmd
- right_prompt_cmd
- complete_ok
- highlight_ok: Whether to perform syntax highlighting
- syntax_check_ok: Whether to perform syntax checking before returning.
- autosuggest_ok: Whether to allow autosuggestions.
- transient_prompt: Whether to reexecute prompt function before final rendering.
- exit_on_interrupt: Whether to exit on interrupt (^C).
- in_silent_mode: If set, do not show what is typed.
- inputfd: The fd for stdin, default to actual stdin.

### State

`CommandlineState`
- text: command line text
- cursor_pos: position of the cursor, may be as large as text.size()
- selection: visual selection
- history
- pager_mode: pager is visible
- pager_fully_disclosed: pager already shows everything if possible
- search_field
- search_mode: pager is visible and search is active
- initialized

## Completion

- src/complete.rs

## Highlight

- src/color.rs
- src/highlight/highlight.rs

## History

- src/history.rs
- src/history/file.rs => `HistoryFileContents`
- src/reader_history_search.rs

## Input (*)

`InputEventQueuer` / `InputData` / `InputEventQueue`

- src/input.rs (key bindings) => `EventQueuePeeker`
- src/input_common.rs => `CharEventType` / `CharEvent` / `KeyInputEvent` / `KeyEvent`
- src/key.rs => `Key` / `Modifiers`

## IO

- src/fd_readable_set.rs
- src/fds.rs

## Killring

- src/kill.rs

## Pager (?)

- src/pager.rs

## Parser

- src/parser.rs

## Screen (*)

- `Screen`
- `Cursor`
- `ScreenData` -> `Line` -> `HighlightedChar`
- `LayoutCache`
- `PromptCacheEntry` -> `PromptLayout`
- src/screen.rs

## Signal

- src/signal.rs => `SigChecker`

## Terminal (*)

- src/term.rs
  - `ColorSupport`
  - `Outputter` / `BufferedOutputter`
  - `Term`
- src/termsize.rs

## Undo / Redo

- src/editable_line.rs

## Width

- src/widecharwidth/

## Tests

## Tools

- src/builtins/fish_key_reader.rs
