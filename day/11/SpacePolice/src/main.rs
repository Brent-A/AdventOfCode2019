use intcode::*;

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


    fn angle(&self) -> f64 {
        let x : f64 = f64::from(self.x);
        let y : f64 = f64::from(self.y);
        let mut a = y.atan2(x) + std::f64::consts::FRAC_PI_2;
        if a > std::f64::consts::PI * 2.0 {
            a = a - std::f64::consts::PI * 2.0;
        }
        if a < 0.0 {
            a = a + std::f64::consts::PI * 2.0;
        }
a

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


#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White = 1,
    Transparent  = 2,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn(self, turn: Value) -> Self {
        match turn {
            Value(0) => { // left
                match self  {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                }
            },
            Value(1) => { // Right
                match self  {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            },
            _ => { panic!("Invalid durn direction: {:?}", turn); },
        }
    }

    fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::Up => Point::new(point.x, point.y - 1),
            Direction::Left => Point::new(point.x - 1, point.y),
            Direction::Down => Point::new(point.x, point.y + 1),
            Direction::Right => Point::new(point.x + 1, point.y),
        }
    }
}

struct Hull {
    tiles: HashMap<Point, Color>
}

fn main() {
    
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program: Vec<Integer> = file.split(",").map(|x| x.parse::<Integer>().unwrap()).collect();

    let mut m = Machine::new(&program);
    
    let mut tiles: HashMap<Point, Color> = HashMap::new();
    let mut robot_location : Point = Point::new(0, 0);
    let mut robot_direction : Direction = Direction::Up;

    tiles.insert(Point::new(0,0), Color::White);
    loop {
        let result = m.run();
        if result.is_ok() {
            break;
        }

        if let Err(e) = result {
            if e == Error::InputNotAvailable {

                loop {
                    let paint_instruction = m.output().as_ref().unwrap().try_recv();
                    if paint_instruction.is_err() {
                        break;
                    }

                    let paint_instruction = paint_instruction.unwrap();
                    match paint_instruction {
                        Value(0) => {
                            tiles.insert(robot_location, Color::Black);
                        },
                        Value(1) => {
                            tiles.insert(robot_location, Color::White);
                        },
                        _ => {
                            panic!("Invalid paint instruction");
                        }
                    }

                    let move_instruction = m.output().as_ref().unwrap().try_recv().unwrap();
                    robot_direction = robot_direction.turn(move_instruction);
                    robot_location = robot_direction.move_point(&robot_location);
                }

                let tile = tiles.get(&robot_location).unwrap_or(&Color::Black);
                match tile {
                    Color::Black => {
                        m.input().send(Value(0)).unwrap();
                    },
                    Color::White => {
                        m.input().send(Value(1)).unwrap();
                    },
                    Color::Transparent => {
                        panic!("Unexpected");
                    }
                }
            }
        }
    }

    let mut upper_left = Point::new(0, 0);
    let mut lower_right = Point::new(0,0);

    for point in tiles.keys() {
        if point.x < upper_left.x {
            upper_left = Point::new(point.x, upper_left.y);
        }
        if point.x > lower_right.x {
            lower_right = Point::new(point.x, lower_right.y);
        }
        if point.y < upper_left.y {
            upper_left = Point::new(upper_left.x, point.y);
        }
        if point.y > lower_right.y {
            lower_right = Point::new(lower_right.x, point.y);
        }
    }

    for y in (upper_left.y)..(lower_right.y+1) {
        for x in (upper_left.x)..(lower_right.x + 1) {
            let color = tiles.get(&Point::new(x, y)).unwrap_or(&Color::Black);
            if *color == Color::White {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    println!("Upper left: {:?}", upper_left);
    println!("Lower right: {:?}", lower_right);

    println!("Painted tiles: {}", tiles.len());

    
}
