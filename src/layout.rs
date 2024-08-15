struct Position {
    col: u16, // first is zero
    row: u16, // first is zero
}

struct Cell {
    idx: usize, // grapheme index
    pos: Position, // relative (at least for row) or absolute
    width: u8, // display width, may not be necessary (only for debug)
}

struct Layout {
    data: String, // optional for input line
    cells: Vec<Cell>,
    styles: Vec<(Range<usize>, Style)>,
}

// Refactor `Highlighter` to style by range
// but try to keep compatibility with old mode by using a library that parse ANSI escape sequence

struct ScreenLayout {
    size: Position, // screen size
    prompts: Vec<Layout>, // prompt [+ optional continuation prompt(s)]
    input_line: Layout,
    cursor: Cell, // relative to ?, no width
    hint: Option<Layout>,
    scroll: Option<Range<usize>>, // when the screen is too small
}

// when prompt is changed -> impact input_line & cursor & hint, shift all the cells
// while scrolling, no call to `Highlighter` / `Hinter` except to highlight matching bracket

impl ScreenLayout {
    // to be used to move the cursor up/down, scroll up/down
    pub fn byte_offset(&self, pos: Position) -> usize {
        // binary search
        let info = self.input_line.cells[?];
        if info.pos == pos {
            return info.idx;
        }
    }

    pub fn position(&self, offset: usize) -> Position {
        // binary search
        let info = self.input_line.cells[?];
        if info.idx == offset {
            return info.pos;
        }
    }

    /// Return `true` if layout is impacted
    pub fn sigwinch(&mut self, size: Position) -> bool {
        if self.size == size {
            return false;
        }
        // check if layout is impacted or not
        // update layout
        self.size = size;
        true
    }
}

impl DeleteListener for ScreenLayout {
    fn start_killing(&mut self) {
    }

    fn delete(&mut self, idx: usize, string: &str, _: Direction) {
        // TODO
    }

    fn stop_killing(&mut self) {
    }
}

impl ChangeListener for ScreenLayout {
    fn insert_char(&mut self, idx: usize, c: char) {
        // TODO
    }

    fn insert_str(&mut self, idx: usize, string: &str) {
        // TODO
    }

    fn replace(&mut self, idx: usize, old: &str, new: &str) {
        // TODO
    }
}
