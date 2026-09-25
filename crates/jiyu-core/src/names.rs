use std::fmt;

pub use non_zero_size::{Size as Index, const_size as const_index, size as index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    index: Index,
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(formatter)
    }
}

pub const PADDING: usize = 3;
pub const FORMAT: &str = "json";

impl Name {
    pub const fn new(index: Index) -> Self {
        Self { index }
    }

    pub const fn index(self) -> Index {
        self.index
    }

    pub fn format(self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{index:0padding$}",
            index = self.index(),
            padding = PADDING
        )
    }

    pub fn file(self) -> String {
        format!("{self}.{FORMAT}")
    }
}

pub const fn name(index: Index) -> Name {
    Name::new(index)
}
