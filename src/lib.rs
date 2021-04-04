#![feature(iter_intersperse)]
#[macro_use]
extern crate error_chain;
use std::collections::BTreeMap;
use std::fmt;

pub mod errors {
    error_chain! {}
}

pub enum JsonnetTypes {
    Null,
    Bool,
    String,
    Number,
    Array,
    Object,
    Fn,
}

pub enum Jsonnet {
    Null,
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<Self>),
    // We only use btreemap instead of hashmap for easily sorting keys.
    Object(BTreeMap<String, Self>),
}

impl fmt::Display for Jsonnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Jsonnet::Null => write!(f, "null"),
            Jsonnet::Bool(x) => write!(f, "{}", x),
            Jsonnet::String(s) => write!(f, "'{}'", s.to_string()),
            Jsonnet::Number(v) => write!(f, "{}", v),
            Jsonnet::Array(xs) => {
                let entries = xs
                    .iter()
                    // indent & trailing commas
                    .map(|x| indent(format!("{},", x), 1))
                    .intersperse("\n".to_string())
                    .collect::<String>();

                write!(f, "[\n{}\n]", entries)
            }
            Jsonnet::Object(o) => {
                let pairs = o
                    .iter()
                    .map(|(k, v)| indent(format!("{}: {},", Jsonnet::String(k.to_string()), v,), 1))
                    .intersperse("\n".to_string())
                    .collect::<String>();
                write!(f, "{{\n{}\n}}", pairs)
            }
        }
    }
}

#[cfg(test)]
mod jsonnet_display_tests {
    use super::*;

    #[test]
    fn renders_null() {
        assert_eq!("null", format!("{}", Jsonnet::Null))
    }

    #[test]
    fn renders_bool() {
        assert_eq!("true", format!("{}", Jsonnet::Bool(true)))
    }

    #[test]
    fn renders_string() {
        assert_eq!("'foo'", format!("{}", Jsonnet::String("foo".to_string())))
    }

    #[test]
    fn renders_number() {
        assert_eq!("12.5", format!("{}", Jsonnet::Number(12.5)))
    }

    #[test]
    fn renders_array() {
        assert_eq!(
            "[
  null,
  false,
  'foo',
  12.5,
]",
            format!(
                "{}",
                Jsonnet::Array(vec![
                    Jsonnet::Null,
                    Jsonnet::Bool(false),
                    Jsonnet::String("foo".to_string()),
                    Jsonnet::Number(12.5),
                ])
            )
        )
    }

    #[test]
    fn renders_object() {
        let mut m = BTreeMap::new();
        m.insert("foo".to_string(), Jsonnet::String("bar".to_string()));
        m.insert("bazz".to_string(), Jsonnet::Null);

        assert_eq!(
            "{
  'bazz': null,
  'foo': 'bar',
}",
            format!("{}", Jsonnet::Object(m))
        )
    }
}

fn indent(x: String, n: usize) -> String {
    x.lines()
        .into_iter()
        // blocks are indented two spaces at a time.
        .map(|s| format!("{indent}{}", s, indent = "  ".repeat(n)))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        let input = "line one
line two

line four";
        assert_eq!(
            "    line one
    line two
    
    line four",
            indent(input.to_string(), 2)
        )
    }
}

pub trait HasSonnet {
    fn jsonnet(&self) -> Jsonnet;
}
