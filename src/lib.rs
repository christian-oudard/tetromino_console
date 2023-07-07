#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};

const ERROR_CHAR: char = '╳';

const BOX_CHARS: &[((u8, u8, u8, u8), char)] = &[
    // (up, right, down, left)
    ((0, 0, 0, 0), ' '),
    ((1, 0, 1, 0), '│'),
    ((1, 0, 1, 1), '┤'),
    ((0, 0, 1, 1), '┐'),
    ((1, 1, 0, 0), '└'),
    ((1, 1, 0, 1), '┴'),
    ((0, 1, 1, 1), '┬'),
    ((1, 1, 1, 0), '├'),
    ((0, 1, 0, 1), '─'),
    ((1, 1, 1, 1), '┼'),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

pub fn double_x(p: &Point) -> Point {
    Point::new(p.x * 2, p.y)
}

pub struct LineGrid(HashMap<(Point, Point), u8>);

impl LineGrid {
    pub fn new() -> LineGrid {
        LineGrid(HashMap::new())
    }

    pub fn line(&mut self, p1: &Point, p2: &Point, n: u8) {
        let (p1, p2) = order_points(p1, p2);
        self.0.insert((*p1, *p2), n);
    }

    pub fn get(&self, p1: &Point, p2: &Point) -> u8 {
        let (p1, p2) = order_points(p1, p2);
        *self.0.get(&(*p1, *p2)).unwrap_or(&0)
    }

    fn keys(&self) -> impl Iterator<Item = &(Point, Point)> {
        self.0.keys()
    }

    pub fn render(&self) -> String {
        // Determine grid bounds.
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        for (p1, p2) in self.keys() {
            min_x = min_x.min(p1.x).min(p2.x);
            max_x = max_x.max(p1.x).max(p2.x);
            min_y = min_y.min(p1.y).min(p2.y);
            max_y = max_y.max(p1.y).max(p2.y);
        }

        // Render grid.
        let mut result = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p0 = point(x, y);
                let mut lines = vec![];
                for dir in DIRECTIONS {
                    let p1 = offset(&p0, &dir);
                    lines.push(self.get(&p0, &p1));
                }
                result.push(boxc(&lines).unwrap_or(ERROR_CHAR));
            }
            result.push('\n');
        }
        result
    }
}

pub struct BoxGrid(HashMap<Point, u8>);

impl BoxGrid {
    pub fn new() -> BoxGrid {
        BoxGrid(HashMap::new())
    }

    pub fn set(&mut self, p: &Point, color: u8) {
        self.0.insert(*p, color);
    }

    pub fn set_shape(&mut self, shape: &Vec<Point>, color: u8) {
        for p in shape {
            self.set(p, color);
        }
    }

    pub fn unset(&mut self, p: &Point) {
        self.0.remove(p);
    }

    pub fn get(&self, p: &Point) -> u8 {
        *self.0.get(p).unwrap_or(&0)
    }

    pub fn render(&self) -> String {
        // Add each position, as well as the positions above and to the left.
        let mut candidate_boxes: HashSet<Point> = HashSet::new();
        for b in self.0.keys() {
            candidate_boxes.insert(*b);
            candidate_boxes.insert(offset(b, &Direction::Up));
            candidate_boxes.insert(offset(b, &Direction::Left));
        }

        // Compare each candidate box with its neighbors to the right and below. If they are
        // different, add an edge.
        let mut line_grid = LineGrid::new();
        for b in candidate_boxes {
            let b_right = offset(&b, &Direction::Right);
            let b_down = offset(&b, &Direction::Down);
            if self.get(&b) != self.get(&b_right) {
                let start = double_x(&b_right);
                let end = offset(&start, &Direction::Down);
                line_grid.line(&start, &end, 1);
            }
            if self.get(&b) != self.get(&b_down) {
                let start = double_x(&b_down);
                let middle = offset(&start, &Direction::Right);
                let end = offset(&middle, &Direction::Right);
                line_grid.line(&start, &middle, 1);
                line_grid.line(&middle, &end, 1);
            }
        }

        line_grid.render()
    }
}

type Shape = Vec<Point>;

pub fn tetrominoes() -> Vec<Shape> {
    vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)], // I
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // O
        vec![(0, 0), (0, 1), (0, 2), (1, 1)], // T
        vec![(0, 0), (0, 1), (0, 2), (1, 0)], // J
        vec![(0, 0), (0, 1), (0, 2), (1, 2)], // L
        vec![(0, 0), (1, 0), (1, 1), (2, 1)], // N
        vec![(0, 0), (0, 1), (1, 1), (1, 2)], // S
    ]
    .into_iter()
    .map(|shape| shape.into_iter().map(|(x, y)| point(x, y)).collect())
    .collect()
}

pub fn rotate_shape(shape: &Shape) -> Shape {
    shape.iter().map(|p| point(-p.y, p.x)).collect()
}

pub fn move_point(p: &Point, offset: &Point) -> Point {
    point(p.x + offset.x, p.y + offset.y)
}

pub fn move_shape(shape: &Shape, offset: &Point) -> Shape {
    shape.iter().map(|p| move_point(p, offset)).collect()
}

pub fn normalize(shape: &Shape) -> Shape {
    // Move the shape so all coordinates are positive.
    let min_x = shape.iter().map(|p| p.x).min().unwrap();
    let min_y = shape.iter().map(|p| p.y).min().unwrap();
    let mut shape = move_shape(shape, &point(-min_x, -min_y));

    // Sort the points in the shape.
    shape.sort();

    shape
}

fn gen_canonical_map() -> HashMap<Shape, Shape> {
    let tets = tetrominoes();
    let mut canonical_tetrominoes = HashMap::new();

    for t in &tets {
        let t_orig = t.clone();
        let mut t = t.clone();
        let mut rotations: Vec<Vec<Point>> = Vec::new();
        for _ in 0..4 {
            t = normalize(&t);
            rotations.push(t.clone());
            t = rotate_shape(&t);
        }

        // The canonical shape is the one which has the lexicographically first coordinate string.
        let canonical = rotations.iter().min().unwrap().clone();
        for rot in rotations {
            canonical_tetrominoes.insert(rot, canonical.clone());
        }
        assert_eq!(t_orig, canonical);
    }

    canonical_tetrominoes
}

lazy_static! {
    static ref CANONICAL_TETROMINOES: HashMap<Shape, Shape> = gen_canonical_map();
}

pub fn canonicalize(shape: &Shape) -> Shape {
    CANONICAL_TETROMINOES.get(&normalize(shape)).unwrap().clone()
}
