# Changes

## External events / user actions

- Screen height / width
- Key pressed
- Mouse
- Signal (kill)

## Internal events / code actions

- Input
- Cursor position / movement
- Selection
- Highlight / style
- Prompt
- Completion
- Hint

## Incremental

- When the screen size is changed, we should not need to invalidate `Highlight`ing / `Hint` / `Prompt` / `Completion`
- When the prompt is change, we should not need to invalidate input `Highlight`ing / `Hint`
- When the cursor is moved, we should not need to invalidate all `Highlight`ing

Or at least we should tell the implementations what kind(s) of change happen(s)
and let them choose what to ignore or not.
