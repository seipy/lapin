use {
    std::ops::Range,
    serde::{Serialize, Deserialize},
    super::*,
};

// note: the std Range type is broken, not Copy and hard
// to pass around and use. It's better to avoid it
// as much as possible.
pub type IntRange = Range<Int>;
pub fn grow_range_to(range: &mut IntRange, i: Int) {
    if i < range.start {
        range.start = i;
    } else if i >= range.end {
        range.end = i + 1;
    }
}

/// A rectangle in the real world
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PosArea {
    pub x: Range<Int>,
    pub y: Range<Int>,
}
impl Clone for PosArea {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl PosArea {
    pub const fn new(x: Range<Int>, y: Range<Int>) -> Self {
        Self { x, y }
    }
    pub const fn empty() -> Self {
        Self {
            x: 0..0,
            y: 0..0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x.len()==0 || self.y.len()==0
    }
    pub const fn from_pos(pos: Pos) -> Self {
        Self {
            x: pos.x..pos.x+1,
            y: pos.y..pos.y+1,
        }
    }
    pub fn contains(&self, pos: Pos) -> bool {
        self.x.contains(&pos.x) && self.y.contains(&pos.y)
    }
    pub fn width(&self) -> Int {
        self.x.end - self.x.start
    }
    pub fn height(&self) -> Int {
        self.y.end - self.y.start
    }
    pub fn center(&self) -> Pos {
        Pos::new(
            (self.x.start + self.x.end)/2,
            (self.y.start + self.y.end)/2,
        )
    }
    /// return the pos in the area which is the nearest one
    /// from the given external pos
    pub fn nearest(&self, mut pos: Pos) -> Pos {
        if pos.x < self.x.start {
            pos.x = self.x.start;
        } else if pos.x >= self.x.end {
            pos.x = self.x.end - 1;
        }
        if pos.y < self.y.start {
            pos.y = self.y.start;
        } else if pos.y >= self.y.end {
            pos.y = self.y.end - 1;
        }
        pos
    }
    /// change the area so that it includes the
    /// passed pos
    pub fn grow_to(&mut self, pos: Pos) {
        grow_range_to(&mut self.x, pos.x);
        grow_range_to(&mut self.y, pos.y);
    }
}

