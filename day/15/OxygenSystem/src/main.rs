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

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];

impl Direction {
    fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::North => Point::new(point.x, point.y - 1),
            Direction::West => Point::new(point.x - 1, point.y),
            Direction::South => Point::new(point.x, point.y + 1),
            Direction::East => Point::new(point.x + 1, point.y),
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
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Unexplored,
    Empty,
    Wall,
    OxygenSystem,
}

struct Map {
    tiles: HashMap<Point, MapTile>,
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

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

    fn get_tile(&self, point: &Point) -> MapTile {
        *self.tiles.get(point).unwrap_or(&MapTile::Unexplored)
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
        self.neighbors(point).iter().filter(|x| {
            x.1 == MapTile::Empty || x.1 == MapTile::OxygenSystem
        }).map(|x| *x).collect()
    }


    fn fill_costs(&self, cost_map: &mut HashMap<Point, i32>, i: i32, point: &Point)
    {
        for (p, t) in self.traversable_neighbors(point) {
            if !cost_map.contains_key(&p) {
                cost_map.insert(p, i);
                self.fill_costs(cost_map, i+1, &p);
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
            let mut options : Vec<(Direction, i32)> = DIRECTIONS.iter().map(|d| {
                let p2 = d.move_point(&p);
                (*d, *cost_map.get(&p2).unwrap_or(&100000))
            }).collect();

            //println!("   options: {:?}", options);
            options.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap() );

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


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MoveResult {
    Wall,
    Moved,
    MovedToOxygen,
}

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

fn draw_map(map: &Map) {

    
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
                MapTile::Empty => { print!(" "); },
                
                MapTile::Unexplored => { print!("?"); },
                
                MapTile::OxygenSystem => { print!("O"); },
                
                MapTile::Wall => { print!("#"); },
            }
        }
        println!("");
    }


}


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
fn main() {
    
    let mut r = Robot::new();
    let mut map = Map::new();

    map.tiles.insert(r.location, MapTile::Empty);

    let mut oxygen : Option<Point> = None;
    loop {

        let unexplored = map.unexplored();
        //println!("Unexplored: {}", unexplored.len());
        if unexplored.len() == 0 {
            break;
        }

        //draw_map(&map);

        //println!("Location: {:?}", r.location);

        let mut path = map.find_path(&r.location, &unexplored[0]);
        let f = path.pop().unwrap();
        // Move to the unexplored region
        for d in path {
            //println!("Moving: {:?}", d);
            let result = r.move_robot(d);
            assert_ne!(result, MoveResult::Wall);
        }

        let projected_position = f.move_point(&r.location);
        
        //println!("Moving*: {:?}", f);
        let result = r.move_robot(f);
        match result {
            MoveResult::Moved => {
                map.tiles.insert(projected_position, MapTile::Empty);
            },
            MoveResult::Wall => {
                map.tiles.insert(projected_position, MapTile::Wall);
            },
            MoveResult::MovedToOxygen => {
                map.tiles.insert(projected_position, MapTile::OxygenSystem);
                oxygen = Some(projected_position);
            }
        }

        if oxygen.is_some() {
            //break;
        }
    }

    
    draw_map(&map);
    
    let path = map.find_path(&Point::new(0,0), &oxygen.unwrap());

    println!("path: {:?}", path);
    println!("pathlen: {}", path.len());

    let mut cost_map = HashMap::new();
    cost_map.insert(oxygen.unwrap(), 0);
    map.fill_costs(&mut cost_map, 1, &oxygen.unwrap());

    let maxval = cost_map.values().max();
    println!("max value: {}", maxval.unwrap());

    //println!("cost_map: {:?}", cost_map);
    //draw_map_costs(&map, &cost_map);
}
