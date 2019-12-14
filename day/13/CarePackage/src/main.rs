use intcode::*;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;

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
        let x: f64 = f64::from(self.x);
        let y: f64 = f64::from(self.y);
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

#[derive(FromPrimitive, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

fn render(tiles: &HashMap<Point, Tile>) {
    let mut upper_left = Point::new(0, 0);
    let mut lower_right = Point::new(0, 0);

    //print!("{}[2J", 27 as char);

    println!("");

    println!("");
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

    let mut block_count = 0;
    for y in (upper_left.y)..(lower_right.y + 1) {
        for x in (upper_left.x)..(lower_right.x + 1) {
            let color = tiles.get(&Point::new(x, y)).unwrap_or(&Tile::Empty);
            match *color {
                Tile::Empty => {
                    print!(" ");
                }
                Tile::Wall => {
                    print!("#");
                }
                Tile::Block => {
                    print!("X");
                    block_count += 1;
                }
                Tile::HorizontalPaddle => {
                    print!("-");
                }
                Tile::Ball => {
                    print!("o");
                }
            }
        }

        println!("");
    }
}

use std::io::Read;

struct GameState {
    inputState: Value,
    lastBall: Point,
    ballDirectionX: i32,
    ballDirectionY: i32,
    ball: Point,
    paddle: Point,
    tiles: HashMap<Point, Tile>,
    score: Integer,
    turn: usize,
}
fn process_output(m: &mut Machine, state: &mut GameState) {
    let mut i = m.output().as_ref().unwrap().try_iter();
    loop {
        if let Some(x) = i.next() {
            let y = i.next().unwrap();
            let tile_type = i.next().unwrap();

            if x.0 == -1 && y.0 == 0 {
                state.score = tile_type.0;
            } else {
                let tile: Tile = FromPrimitive::from_i64(tile_type.0).unwrap();

                if tile == Tile::Ball {
                    state.ball = Point::new(x.0.try_into().unwrap(), y.0.try_into().unwrap());

                    if state.ball.x < state.lastBall.x {
                        state.ballDirectionX = -1;
                    } else if state.ball.x > state.lastBall.x {
                        state.ballDirectionX = 1;
                    }

                    if state.ball.y < state.lastBall.y {
                        state.ballDirectionY = -1;
                    } else if state.ball.y > state.lastBall.y {
                        state.ballDirectionY = 1;
                    }

                    state.lastBall = state.ball;
                } else if tile == Tile::HorizontalPaddle {
                    state.paddle =
                        Point::new(x.0.try_into().unwrap(), y.0.try_into().unwrap());
                }

                state.tiles.insert(
                    Point::new(x.0.try_into().unwrap(), y.0.try_into().unwrap()),
                    tile,
                );
            }
        } else {
            break;
        }
    }

    let blocks = state.tiles.values().filter(|x| **x == Tile::Block ).count();
    

    let mut deltay = state.paddle.y - state.ball.y - 1;
    if state.ballDirectionY == -1 {
        deltay = deltay * 2;
    }
    let mut predictedtarget = state.ball.x + deltay * state.ballDirectionX;
    
    if predictedtarget < state.paddle.x {
        state.inputState = Value(-1);
    } else if predictedtarget > state.paddle.x {
        state.inputState = Value(1);
    } else {
        state.inputState = Value(0);
    }

    if (state.turn < 20 || blocks < 3) {
        render(&state.tiles);
        println!(
            "Score: {} Turn: {} Blocks: {}, InputState: {}    Ball: {:?}   Paddle: {:?}  Predicted: {}",
            state.score, state.turn, blocks, state.inputState.0, state.ball, state.paddle, predictedtarget
        );
    }

    state.turn = state.turn + 1;


}
fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut program: Vec<Integer> = file
        .split(",")
        .map(|x| x.parse::<Integer>().unwrap())
        .collect();

    program[0] = 2;

    let mut m = Machine::new(&program);


    let mut state = GameState {
        tiles: HashMap::new(),
        score: 0,
        inputState: Value(0),
        lastBall: Point::new(0, 4),
        ballDirectionX: 0,
        ballDirectionY: 0,
        ball: Point::new(0,0),
        paddle: Point::new(0, 0),
        turn: 0,
    };

    loop {
        match m.run() {
            Err(Error::InputNotAvailable) => {
                
                process_output(&mut m, &mut state);

                m.input().send(state.inputState).unwrap();
            }
            Err(e) => {
                panic!(e);
            }
            Ok(_) => {
                
                process_output(&mut m, &mut state);

                break;
            }
        }

    }
}
