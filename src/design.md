# Design

The current API is not fitted for:
- scrolling (screen height < input height)
- continuation prompt (specific text displayed before [2nd, ...] lines)
- minimal refresh

### [Highlighting](https://docs.rs/rustyline/*/rustyline/highlight/trait.Highlighter.html)

Currently, the `Highlighter` returns directly a styled (ansi escape sequence) text for the whole input lines.
But for continuation prompt or scrolling we need to split the input by line (soft wrapped or not).
Which means that we need `[(Range,Style)]` instead of `Cow<str>`.
Where `Range` refers to input indexes.

I guess we can keep the old `Highlighter` API by parsing the styled text (TODO: validate).
See https://github.com/doy/vt100-rust or jline3 AnsiWriter
And add new methods that returns `Vec<(Range,Style)>` (TODO: validate).
Something like [isocline](https://github.com/daanx/isocline/blob/c9310ae58941559d761fe5d2dd2713d245f18da6/include/isocline.h#L284)

And add a dedicated method for continuation prompt.

### Layout cache

Currently, `rustyline` repaints the whole input lines for each keystroke.
Except when pushing chars at the end of input or moving the cursor.
And each time we do a whole repaint, the `Highlighter` is invoked.

But while scrolling, we want to skip the `Highlighter` (except to highlight matching bracket when cursor is moved).
Which means we need to remember the styles.
Also, we don't want to recompute the line breaks.
...

I guess we can use a dedicated [ChangeListener](https://docs.rs/rustyline/*/rustyline/line_buffer/trait.ChangeListener.html) to invalidate / refresh the layout cache.

See old draft [here](https://github.com/kkawakam/rustyline/compare/master...gwenn:rustyline:redraw)
...