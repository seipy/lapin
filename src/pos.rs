use {
    anyhow::Result,
    crate::{
        io::W,
    },
    crossterm::{
        cursor,
        QueueableCommand,
    },
    termimad::{
        Area,
    },
};

pub type Int = i32;

// a position in the real world (the one full of rabbits and wolves)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: Int,
    pub y: Int,
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    RightDown,
    DownLeft,
    LeftUp
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }
    pub fn center_of(area: &Area) -> Self {
        Self {
            x: (area.left+area.width/2) as Int,
            y: (area.top+area.height/2) as Int,
        }
    }
    pub fn in_grid(self, width: Int, height: Int) -> bool {
        self.x>=0 && self.y>=0 && self.x<width && self.y<height
    }
    pub fn mh_distance(a: Pos, b: Pos) -> Int {
        (a.x-b.x).abs().max((a.y-b.y).abs())
    }
    pub fn manhattan_distance(a: Pos, b: Pos) -> Int {
        (a.x-b.x).abs() + (a.y-b.y).abs()
    }
    /// return the first direction to follow on a path
    /// (or none if we're yet on destination or if the
    /// path doesn't starts from there)
    pub fn first_dir(&self, path: &Vec<Pos>) -> Option<Dir> {
        path.get(0).and_then(|dst| self.dir_to(*dst))
    }
    /// return the direction to follow to directly reach
    /// the dst. Return None if the other pos isn't a
    /// direct neighbour.
    pub fn dir_to(&self, dst: Pos) -> Option<Dir> {
        match (dst.x-self.x, dst.y-self.y) {
            (0, -1) => Some(Dir::Up),
            (1, 0)  => Some(Dir::Right),
            (0, 1)  => Some(Dir::Down),
            (-1, 0) => Some(Dir::Left),
            (1, -1) => Some(Dir::UpRight),
            (1, 1)  => Some(Dir::RightDown),
            (-1, 1)  => Some(Dir::DownLeft),
            (-1, -1) => Some(Dir::LeftUp),
            _ => None,
        }
    }
    pub fn in_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Pos { x:self.x, y:self.y-1 },
            Dir::Right => Pos { x:self.x+1, y:self.y },
            Dir::Down => Pos { x:self.x, y:self.y+1 },
            Dir::Left => Pos { x:self.x-1, y:self.y },
            Dir::UpRight => Pos { x:self.x+1, y:self.y-1 },
            Dir::RightDown => Pos { x:self.x+1, y:self.y+1 },
            Dir::DownLeft => Pos { x:self.x-1, y:self.y+1 },
            Dir::LeftUp => Pos { x:self.x-1, y:self.y-1 },
        }
    }
}

impl ScreenPos {
    pub fn new(x: u16, y:u16) -> Self {
        Self { x, y }
    }
    pub fn goto(self, w: &mut W) -> Result<()> {
        w.queue(cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }
}

pub trait Mobile {
    fn get_pos(&self) -> Pos;
    fn set_pos(&mut self, pos: Pos) -> Pos;
}
