# Syntax

* Make possible to define a tokenizer/parser.
* Parser will be invoked when the line is modified.
* Make possible to do incremental parsing by providing the change (see tree-sitter).

```rust
trait Parser {
    type Document;
    fn update(&mut self, line: &str, change: InputEdit) -> Self::Document;
}
```
* update `Helper`/`Completer`/`Highlighter`/`Hinter` accordingly
```
pub trait Helper
where
    Self: Completer,
    Self: Hinter,
    Self: Highlighter,
    Self: Parser
{
    type Document; // How to make sure all traits are coherent/have the same Document type ?
}

pub trait Completer {
    type Document;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
        doc: &Document,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
...
    }
...
}

pub trait Highlighter {
    type Document;

    fn highlight<'l>(&self, line: &'l str, pos: usize, doc: &Document) -> Cow<'l, str> {
...
    }
...
    fn highlight_char(&self, line: &str, pos: usize, doc: &Document) -> bool {
...
    }
}

pub trait Hinter {
    type Document;

    fn hint(&self, line: &str, pos: usize, doc: &Document, ctx: &Context<'_>) -> Option<String> {
    }
}
```

