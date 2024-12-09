use std::{ops::Deref, str::Lines};

use anyhow::{Context, Result};

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub struct NumberLineParser<'a> {
    lines: Lines<'a>,
    split_fn: Box<dyn Fn(&'a str) -> Box<dyn Iterator<Item = &'a str> + 'a>>,
}

impl<'a> NumberLineParser<'a> {
    pub fn split_comma(s: &'a str) -> Self {
        Self::new(s, |line| Box::new(line.split(',')))
    }

    pub fn split_whitespace(s: &'a str) -> Self {
        Self::new(s, |line| Box::new(line.split_whitespace()))
    }

    fn new(
        s: &'a str,
        split_fn: impl Fn(&'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> + 'static,
    ) -> Self {
        Self {
            lines: s.lines(),
            split_fn: Box::new(split_fn),
        }
    }
}

impl Iterator for NumberLineParser<'_> {
    type Item = Result<Vec<u32>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            self.split_fn.deref()(line)
                .map(|x| x.parse().context(format!("Failed to parse: '{}'", x)))
                .collect::<Result<Vec<_>>>()
        })
    }
}
