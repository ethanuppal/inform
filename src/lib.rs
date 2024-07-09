pub mod fmt;

#[cfg(test)]
mod tests {
    use std::fmt::{self, Display, Write};

    use crate::fmt::IndentFormatter;

    struct TestIndentFormatter;
    impl Display for TestIndentFormatter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
}
