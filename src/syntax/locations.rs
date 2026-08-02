#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

impl Point {
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Range {
    pub start: Point,
    pub end: Point,
}

impl Range {
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}
