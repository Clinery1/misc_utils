use serde::{Serialize, Deserialize};
use std::{
    ops::{
        Add,
        Range,
        RangeInclusive,
    },
    cmp::Ordering,
};


pub mod sparse_list;
pub mod keyed_vec;
pub mod slotmap;
pub mod stack;


#[macro_export]
macro_rules! define_keys {
    ($first:ident $(,$name:ident)*)=>{
        define_keys!($first, $($name,)*);
    };

    ($($name:ident,)*)=>{
        $(
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
            pub struct $name(pub usize);
            impl $crate::Key for $name {
                fn from_id(id: usize)->Self {$name(id)}
                fn id(&self)->usize {self.0}
            }
            impl $name {
                pub fn invalid()->Self {Self(usize::MAX)}
            }
        )*
    };
}


pub trait Key {
    fn from_id(id: usize)->Self;
    fn id(&self)->usize;
}
// could be useful, but probably not.
impl Key for usize {
    fn from_id(id: usize)->Self {id}
    fn id(&self)->usize {*self}
}


/// A range. Basically [`Range`], but impements [`Copy`] and only uses [`usize`]
#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Span {
    /// Inclusive
    pub start: usize,
    /// Exclusive
    pub end: usize,
}
impl From<Range<usize>> for Span {
    fn from(r: Range<usize>)->Self {
        Span {
            start: r.start,
            end: r.end,
        }
    }
}
impl From<RangeInclusive<usize>> for Span {
    fn from(r: RangeInclusive<usize>)->Self {
        Span {
            start: *r.start(),
            end: r.end() + 1,
        }
    }
}
impl Span {
    pub fn contains(&self, i: usize)->bool {
        i >= self.start && i < self.end
    }
}

/// Line and column are zero-based
#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    pub span: Span,
    pub line: usize,
    pub end_line: usize,
    pub column: usize,
    pub end_column: usize,
}
impl Add for Location {
    type Output = Self;
    fn add(self, other: Self)->Self {
        let span_start = self.span.start.min(other.span.start);
        let span_end = self.span.end.min(other.span.end);

        let line;
        let column;
        if self.line < other.line {
            line = self.line;
            column = self.column;
        } else {
            line = other.line;
            column = other.column;
        }

        let end_line;
        let end_column;
        if self.end_line > other.end_line {
            end_line = self.end_line;
            end_column = self.end_column;
        } else {
            end_line = other.end_line;
            end_column = other.end_column;
        }

        Location {
            span: Span(span_start, span_end),
            line,
            end_line,
            column,
            end_column,
        }
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, o: &Self)->Option<Ordering> {
        if self.line == o.line {
            return self.column.partial_cmp(&o.column);
        }
        return self.line.partial_cmp(&o.line);
    }
}

/// Allows converting between source index spans and location spans
pub struct SpanConverter {
    line_spans: Vec<Span>,
}
impl SpanConverter {
    pub fn new(source: &str)->Self {
        let mut line_spans = Vec::new();
        let mut prev_start = 0;

        for (i, c) in source.char_indices() {
            match c {
                '\n'=>{
                    line_spans.push(Span(prev_start, i + 1));
                    prev_start = i + 1;
                },
                _=>{},
            }
        }
        line_spans.push(Span(prev_start, source.len()));

        SpanConverter {
            line_spans,
        }
    }

    /// Converts a Span to a LocationSpan
    pub fn convert(&self, span: Span)->Location {
        let mut start = None;
        let mut end = None;
        for (i, line_span) in self.line_spans.iter().enumerate() {
            if line_span.contains(span.start) {
                start = Some((i, span.start - line_span.start));
            }
            if line_span.contains(span.end) {
                end = Some((i, span.end - line_span.start));

                break;
            }
        }

        let start = start.unwrap();
        let end = end.unwrap();

        return Location {
            span,
            line: start.0,
            end_line: end.0,
            column: start.1,
            end_column: end.1,
        };
    }
}


/// Convenient way to allow for a `Span(start, end)` constructor, but still have `span.start` and
/// `span.end` fields.
#[allow(non_snake_case)]
pub fn Span(start: usize, end: usize)->Span {
    Span {start, end}
}
