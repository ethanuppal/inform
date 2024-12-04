<div align="center">
 <img src="./assets/inform-logo.svg" width="250px">
 <p><strong>indent formatting, everywhere</strong></p>
</div>

![CI Badge](https://github.com/ethanuppal/inform/actions/workflows/ci.yaml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/inform)
![docs.rs](https://img.shields.io/docsrs/inform)
![Crates.io License](https://img.shields.io/crates/l/inform)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`inform` gives you

- A `std::fmt::Formatter` drop-in replacement designed for formatting
  structured data such as AST nodes.
- More generally, an API for formatting any type implementing
  `std::io::Write` or `std::fmt::Write` with indentation.

The format and I/O implementations are behind [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) `"fmt"` and `"io"` respectively, both of which are enabled by default.

## Contents

- [Examples](#examples)
- [Projects using `inform`](#projects-using-inform)
- [Alternatives](#alternatives)
- [License](#license)

<a name="examples"></a>

## Examples

Here's how you can use `fmt::IndentFormatter`:

```rs
use std::fmt::{self, Write};
use inform::common::IndentWriterCommon, fmt::IndentFormatter;

struct TestIndentFormatter;

impl fmt::Display for TestIndentFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

Here's how you can use `fmt::IndentWriter`:

```rs
use std::fmt::{self, Write};
use inform::common::IndentWriterCommon, fmt::IndentWriter;

fn write_text(str: &mut String) -> fmt::Result {
    let mut f = IndentWriter::new(str, 2);
    writeln!(f, "hello\ngoodbye")?;
    f.increase_indent();
    writeln!(f, "hello\ngoodbye")?;
    f.decrease_indent();
    writeln!(f, "hello\ngoodbye")
}

#[test]
fn test_indent_writer() {
    let mut buffer = String::new();
    write_text(&mut buffer).expect("failed to format");
    assert_eq!(
        "hello\ngoodbye\n  hello\n  goodbye\nhello\ngoodbye\n",
        buffer
    );
}
```

<a name="projects-using-inform"></a>

## Projects using `inform`

- [`calyx-writer`](https://github.com/calyxir/calyx/tree/main/tools/calyx-writer)
- [`spadefmt`](https://github.com/ethanuppal/spadefmt)
- [`pulsar`](https://github.com/ethanuppal/pulsar/tree/main)

<a name="alternatives"></a>

## Alternatives

The following crates are alternatives that I found did not fit my use case.

- <https://crates.io/crates/indent>
- <https://crates.io/crates/indenter>
- <https://crates.io/crates/indentation>
- <https://docs.rs/indent_write/latest/indent_write/index.html>

<a name="license"></a>

## License

A copy of the LGPL License is provided in the [LICENSE](LICENSE) file.