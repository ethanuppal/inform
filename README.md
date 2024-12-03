# inform: INdent FORMatter

A `std::fmt::Formatter` drop-in replacement designed for formatting structured data such as AST nodes.

```rs
use std::fmt::{self, Display, Write};

use inform::fmt::IndentFormatter;

struct TestIndentFormatter;
impl Display for TestIndentFormatter {
    fn fmt(&self, f: &mut fmt::Formatter -> fmt::Result {
        let mut f = IndentFormatter::new(f, 2);
        writeln!(f, "hello\ngoodbye")?;
        f.increase_indent();
        writeln!(f, "hello\ngoodbye")?;
        f.decrease_indent();
        writeln!(f, "hello\ngoodbye")
    }
}

#[test]
fn test_indent_formatter() {
    assert_eq!(
        "hello\ngoodbye\n  hello\n  goodbye\nhello\ngoodbye\n",
        TestIndentFormatter.to_string()
    );
}
```

## Alternatives

The following crates are alternatives that I found did not fit my use case.

- <https://crates.io/crates/indent>
- <https://crates.io/crates/indenter>
- <https://crates.io/crates/indentation>
- <https://docs.rs/indent_write/latest/indent_write/index.html>

## License

A copy of the MIT License is provided in the [LICENSE](LICENSE) file.
