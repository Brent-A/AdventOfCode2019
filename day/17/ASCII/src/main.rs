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

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
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

impl Turn {

}

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

    fn get_turn(&self, o: &Self) -> Vec<Turn> {
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
    fn turn (&self, t: Turn) -> Self {
        match t {
            Turn::Left => {
                match self {
                    Self::North => Self::West,
                    Self::West => Self::South,
                    Self::South => Self::East,
                    Self::East => Self::North,
                }
            },
            Turn::Right => {
                match self {
                    Self::North => Self::East,
                    Self::East => Self::South,
                    Self::South => Self::West,
                    Self::West => Self::North,
                }
            }
        }
    }

    fn value(&self) -> Value {
        match self {
            Direction::North => Value(1),
            Direction::South => Value(2),
            Direction::West => Value(3),
            Direction::East => Value(4),
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Space,
    Scaffold,
    Robot(Direction),
}

impl MapTile {
    fn from_ascii(c: char) -> MapTile {
        match c {
            '.' => MapTile::Space,
            '#' => MapTile::Scaffold,
            c => MapTile::Robot(Direction::from_ascii(c)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            MapTile::Space => '.',
            MapTile::Scaffold => '#',
            MapTile::Robot(d) => d.to_char(),
        }
    }

    fn traversable(&self) -> bool {
        match self {
            MapTile::Space => false,
            MapTile::Scaffold => true,
            MapTile::Robot(_) => true,
        }
    }
}

struct Map {
    tiles: HashMap<Point, MapTile>,
    robot: (Point, Direction),
}

impl Map {
    fn new(chars : &[char]) -> Self {
        let mut s = Self {
            tiles: HashMap::new(),
            robot: (Point::new(0, 0), Direction::North)
        };

        //println!("chars: {:?}", chars);
        let mut p = Point::new(0, 0);
        for c in chars {
            //println!("{:?} '{}'", p, c);
            if *c == '\n' {
                p = Point::new(0, p.y + 1);
            }
            else {
                let tile = MapTile::from_ascii(*c);
                s.tiles.insert(p, tile);
                
                p = Point::new(p.x + 1, p.y);
            }
        }

        s
    }


    /*
    fn unexplored(&self) -> Vec<Point> {
        let mut v = Vec::new();
        for point in self.tiles.iter().filter_map(|(k, v)| {if *v == MapTile::Empty || *v == MapTile::OxygenSystem { return Some(k) } else { return None } }) {

            for d in &DIRECTIONS {
                let p2 = d.move_point(&point);
                if self.get_tile(&p2) == MapTile::Unexplored {
                    v.push(p2);
                }
            }
        }
        v
    }
    */

    fn intersections(&self) -> Vec<Point> {
        self.tiles.iter().filter(|(p, t)| t.traversable() && self.traversable_neighbors(p).len() == 4).map(|(p,_)| *p).collect()
    }
    fn get_tile(&self, point: &Point) -> MapTile {
        *self.tiles.get(point).unwrap_or(&MapTile::Space)
    }

    fn neighbors(&self, point: &Point) -> Vec<(Point, MapTile)> {
        let mut v = Vec::new();
        for d in &DIRECTIONS {
            let p2 = d.move_point(&point);
            v.push((p2, self.get_tile(&p2)));
        }
        v
    }

    fn traversable_neighbors(&self, point: &Point) -> Vec<(Point, MapTile)> {
        self.neighbors(point)
            .iter()
            .filter(|x| match x.1 {
                MapTile::Scaffold => true,
                MapTile::Robot(_) => true,
                MapTile::Space => false,
            })
            .map(|x| *x)
            .collect()
    }

    fn fill_costs(&self, cost_map: &mut HashMap<Point, i32>, i: i32, point: &Point) {
        for (p, t) in self.traversable_neighbors(point) {
            if !cost_map.contains_key(&p) {
                cost_map.insert(p, i);
                self.fill_costs(cost_map, i + 1, &p);
            }
        }
    }

    fn find_path(&self, start: &Point, end: &Point) -> Vec<Direction> {
        let mut cost_map = HashMap::new();

        //println!(" finding path from {:?} to {:?}", start, end);
        //println!(" finding costs");
        cost_map.insert(*end, 0);
        self.fill_costs(&mut cost_map, 1, end);

        //println!(" calculated {} points", cost_map.len());
        //println!(" finding path");
        let mut directions = Vec::new();
        let mut p = *start;
        while p != *end {
            //println!("  p: {:?}", p);
            let mut options: Vec<(Direction, i32)> = DIRECTIONS
                .iter()
                .map(|d| {
                    let p2 = d.move_point(&p);
                    (*d, *cost_map.get(&p2).unwrap_or(&100000))
                })
                .collect();

            //println!("   options: {:?}", options);
            options.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let d = options[0].0;
            p = d.move_point(&p);
            directions.push(d);
        }
        directions
    }
}

struct Robot {
    m: Machine,
    location: Point,
}

/*

impl Robot {
    fn new() -> Self {

        let file = std::fs::read_to_string("input.txt").unwrap();
        let program: Vec<Integer> = file.split(",").map(|x| x.parse::<Integer>().unwrap()).collect();

        let mut m = Machine::new(&program);

        Self {
            m: m,
            location: Point::new(0, 0),
        }
    }

    fn move_robot(&mut self, dir: Direction) -> MoveResult{
        self.m.input().send(dir.value()).unwrap();

        let result = self.m.run();
        match result {
            Err(e) if e == Error::InputNotAvailable => {
                //expected
            },
            _ => { panic!("Unexpected run result: {:?}", result); }
        }

        let output = self.m.output().as_ref().unwrap().recv().unwrap();

        match output {
            Value(0) => MoveResult::Wall,
            Value(1) => {
                self.location = dir.move_point(&self.location);
                MoveResult::Moved
            },
            Value(2) => {
                self.location = dir.move_point(&self.location);
                MoveResult::MovedToOxygen
            }
            _ => { panic!("Unexpected output: {:?}", output); }
        }
    }

}
*/

fn draw_map(map: &Map) {
    let mut upper_left = Point::new(0, 0);
    let mut lower_right = Point::new(0, 0);

    for point in map.tiles.keys() {
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

    for y in (upper_left.y)..(lower_right.y + 1) {
        for x in (upper_left.x)..(lower_right.x + 1) {
            let tile = map.get_tile(&Point::new(x, y));
            match tile {
                MapTile::Space => {
                    print!(" ");
                }
                MapTile::Scaffold => {
                    print!("X");
                }
                MapTile::Robot(d) => {
                    print!("{}", d.to_char());
                }
            }
        }
        println!("");
    }
}

/*
fn draw_map_costs(map: &Map, cost_map: &HashMap<Point, i32>) {


    let mut upper_left = Point::new(0, 0);
    let mut lower_right = Point::new(0,0);

    for point in map.tiles.keys() {
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
            let tile = map.get_tile(&Point::new(x,y));
            match tile {
                MapTile::Empty => {
                    let v = cost_map.get(&Point::new(x,y)).unwrap();
                    print!("{:#03}", v);
                 },

                MapTile::Unexplored => { print!(" ? "); },

                MapTile::OxygenSystem => { print!(" O "); },

                MapTile::Wall => { print!("###"); },
            }
        }
        println!("");
    }


}
*/

struct ASCII {
    program: Vec<Integer>,
}

impl ASCII {
    fn new() -> Self {
        let file = std::fs::read_to_string("input.txt").unwrap();
        let program: Vec<Integer> = file
            .split(",")
            .map(|x| x.parse::<Integer>().unwrap())
            .collect();

        Self { program: program }
    }

    fn get_map(&self) -> Map {
        let mut machine = Machine::new(&self.program);

        machine.run().unwrap();

        
        let chars : Vec<char> = machine.output().as_ref().unwrap().try_iter().map(|x| std::char::from_u32(x.0.try_into().unwrap()).unwrap() ).collect();

        let mut map = Map::new(&chars);
        
        map

    }

    fn run(&self, main: &str, A: &str, B: &str, C: &str) {
        let mut program = self.program.clone();
        program[0] = 2;
        let mut machine = Machine::new(&program);

        for c in main.chars() {
            machine.input().send(Value(c as i64));
        }
        machine.input().send(Value(10));

        
        for c in A.chars() {
            machine.input().send(Value(c as i64));
        }
        machine.input().send(Value(10));

        
        for c in B.chars() {
            machine.input().send(Value(c as i64));
        }
        machine.input().send(Value(10));

        
        for c in C.chars() {
            machine.input().send(Value(c as i64));
        }
        machine.input().send(Value(10));

        // feed
        machine.input().send(Value('n' as i64));
        machine.input().send(Value(10));

        machine.run().unwrap();

        let output : Vec<Value> = machine.output().as_ref().unwrap().try_iter().collect();
   
        let chars : Vec<char> = output.iter().map(|x| std::char::from_u32(x.0.try_into().unwrap()).unwrap() ).collect();

        for c in chars {
            if c.is_ascii() {
            print!("{}", c);
            }
            else {
                println!("{}", c as i64);
            }
        }
        //let map = Map::new(&chars);
        //draw_map(&map);
        //println!("output: {:?}", output);
    }
}

fn main() {

    let mut a = ASCII::new();

    let map = a.get_map();

    draw_map(&map);

    let alignment : i32 = map.intersections().iter().map(|x| x.x * x.y).sum();
    println!("alignment: {}", alignment);

    let A = "R,6,R,6,R,8,L,10,L,4";
    let B = "L,4,L,12,R,6,L,10";
    let C = "R,6,L,10,R,8";
    let MAIN = "A,C,C,A,B,A,B,A,B,C";

    a.run(&MAIN, &A, &B, &C);

    

}
