# Layout

For correctness, simplicity and performance,
we need to improve the logic to refresh the screen.

* Remove/move logic into a dedicated layer between `State` and `Renderer`
* Invalidate/repaint only the impacted line(s)/cells.
* Hotspots:
```
Sort by top of stack, same collapsed (when >= 5):
        unicode_segmentation::tables::grapheme::grapheme_category::h7c3f67445534791a  (in example)        329
        unicode_segmentation::grapheme::GraphemeCursor::next_boundary::he29d337853eeb707  (in example)        115
        _$LT$rustyline..tty..unix..PosixRenderer$u20$as$u20$rustyline..tty..Renderer$GT$::calculate_position::h27c3e32aa6d92973  (in example)        106
```

Check <https://github.com/fish-shell/fish-shell/blob/master/src/screen.rs> implementation.

## Old Draft

<https://github.com/kkawakam/rustyline/compare/master...gwenn:rustyline:redraw#diff-5caf3cc5d3186459b336694e55ad1898aaf48294956d7886e318cde96bb9eb99>
