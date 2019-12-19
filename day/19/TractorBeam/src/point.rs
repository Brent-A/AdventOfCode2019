use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn move(&self, direction: &Direction) -> Self {
        direction.move_point(&self)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;
    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for &Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

impl Turn {}

#[derive(PartialEq, Debug, Copy, Clone, Eq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::North => Point::new(point.x, point.y - 1),
            Direction::West => Point::new(point.x - 1, point.y),
            Direction::South => Point::new(point.x, point.y + 1),
            Direction::East => Point::new(point.x + 1, point.y),
        }
    }

    fn get_turns(&self, o: &Self) -> Vec<Turn> {
        match self {
            Self::North => match o {
                Self::North => vec![],
                Self::West => vec![Turn::Left],
                Self::South => vec![Turn::Left, Turn::Left],
                Self::East => vec![Turn::Right],
            },
            Self::West => match o {
                Self::West => vec![],
                Self::South => vec![Turn::Left],
                Self::East => vec![Turn::Left, Turn::Left],
                Self::North => vec![Turn::Right],
            },
            Self::South => match o {
                Self::South => vec![],
                Self::East => vec![Turn::Left],
                Self::North => vec![Turn::Left, Turn::Left],
                Self::West => vec![Turn::Right],
            },
            Self::East => match o {
                Self::East => vec![],
                Self::North => vec![Turn::Left],
                Self::West => vec![Turn::Left, Turn::Left],
                Self::South => vec![Turn::Right],
            },
        }
    }
    
    fn turn(&self, t: Turn) -> Self {
        match t {
            Turn::Left => match self {
                Self::North => Self::West,
                Self::West => Self::South,
                Self::South => Self::East,
                Self::East => Self::North,
            },
            Turn::Right => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
        }
    }

    fn from_ascii(c: char) -> Direction {
        match c {
            '<' => Direction::West,
            '^' => Direction::North,
            'V' => Direction::South,
            'v' => Direction::South,
            '>' => Direction::East,
            _ => {
                panic!("Invalid direction character: {}", c);
            }
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::West => '<',
            Direction::North => '^',
            Direction::South => 'V',
            Direction::East => '>',
        }
    }
}
