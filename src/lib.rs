use std::collections::HashMap;

const BOX_CHARS: &[((u8, u8, u8, u8), char)] = &[
    // (up, right, down, left)
    ((0, 0, 0, 0), ' '),
    ((1, 0, 1, 0), '│'),
    ((1, 0, 1, 1), '┤'),
    ((1, 0, 1, 0), '╡'),
    ((2, 0, 2, 1), '╢'),
    ((0, 0, 2, 1), '╖'),
    ((0, 0, 1, 2), '╕'),
    ((2, 0, 2, 2), '╣'),
    ((2, 0, 2, 0), '║'),
    ((0, 0, 2, 2), '╗'),
    ((2, 0, 0, 2), '╝'),
    ((2, 0, 0, 1), '╜'),
    ((1, 0, 0, 2), '╛'),
    ((0, 0, 1, 1), '┐'),
    ((1, 1, 0, 0), '└'),
    ((1, 1, 0, 1), '┴'),
    ((0, 1, 1, 1), '┬'),
    ((1, 1, 1, 0), '├'),
    ((0, 1, 0, 1), '─'),
    ((1, 1, 1, 1), '┼'),
    ((1, 2, 1, 0), '╞'),
    ((2, 1, 2, 0), '╟'),
    ((2, 2, 0, 0), '╚'),
    ((0, 2, 2, 0), '╔'),
    ((2, 2, 0, 2), '╩'),
    ((0, 2, 2, 2), '╦'),
    ((2, 2, 2, 0), '╠'),
    ((0, 2, 0, 2), '═'),
    ((2, 2, 2, 2), '╬'),
    ((1, 2, 0, 2), '╧'),
    ((2, 1, 0, 1), '╨'),
    ((0, 2, 1, 2), '╤'),
    ((0, 1, 2, 1), '╥'),
    ((2, 1, 0, 0), '╙'),
    ((1, 2, 0, 0), '╘'),
    ((0, 2, 1, 0), '╒'),
    ((0, 1, 2, 0), '╓'),
    ((2, 1, 2, 1), '╫'),
    ((1, 2, 1, 2), '╪'),
    ((1, 0, 0, 1), '┘'),
    ((0, 1, 1, 0), '┌'),
    ((0, 0, 0, 1), '╴'),
    ((1, 0, 0, 0), '╵'),
    ((0, 1, 0, 0), '╶'),
    ((0, 0, 1, 0), '╷'),
];

pub fn boxc(lines: &Vec<u8>) -> Option<char> {
    if lines.len() != 4 {
        return None;
    }
    let key = (lines[0], lines[1], lines[2], lines[3]);
    BOX_CHARS
        .iter()
        .find_map(|&(k, c)| if k == key { Some(c) } else { None })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

fn order_points<'a>(p1: &'a Point, p2: &'a Point) -> (&'a Point, &'a Point) {
    if p1.x < p2.x || p1.y < p2.y {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub const DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

pub fn offset(p: &Point, d: &Direction) -> Point {
    match d {
        Direction::Up => Point::new(p.x, p.y - 1),
        Direction::Right => Point::new(p.x + 1, p.y),
        Direction::Down => Point::new(p.x, p.y + 1),
        Direction::Left => Point::new(p.x - 1, p.y),
    }
}

pub struct Grid(HashMap<(Point, Point), u8>);

impl Grid {
    pub fn new() -> Grid {
        Grid(HashMap::new())
    }

    pub fn line(&mut self, p1: &Point, p2: &Point, n: u8) {
        let (p1, p2) = order_points(p1, p2);
        self.0.insert((*p1, *p2), n);
    }

    pub fn get(&self, p1: &Point, p2: &Point) -> u8 {
        let (p1, p2) = order_points(p1, p2);
        *self.0.get(&(*p1, *p2)).unwrap_or(&0)
    }

    pub fn keys(&self) -> impl Iterator<Item = &(Point, Point)> {
        self.0.keys()
    }
}
