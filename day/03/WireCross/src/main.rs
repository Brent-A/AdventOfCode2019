
mod test {
    use super::*;

    #[test]
    fn parse_dir() {
        assert_eq!(Direction::Up, get_direction('U'));
    }

    #[test]
    fn parse_segment() {
        assert_eq!(Segment { direction: Direction::Up, distance: 10}, 
            Segment::parse("U10"));
    }

    #[test]
    fn parse_wire() {
        let wire = Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        println!("{:?}", wire);
    }

    #[test]
    fn example0() {
        let w1= Wire::parse("R8,U5,L5,D3");
        let w2= Wire::parse("U7,R6,D4,L4");
        
        let mut grid = Grid::new();
        grid.add_wire(&w1);
        grid.add_wire(&w2);

        println!("intersections: {:?}", grid.intersections);

        let nearest = grid.get_nearest_intersection();
        println!("nearest: {:?}", nearest);
        let cheapest = grid.get_cheapest_intersection();
        println!("cheapest: {:?}", cheapest);

        assert_eq!(nearest.distance(), 6);
        assert_eq!(grid.get_total_steps(&cheapest), 30);
    }
    #[test]
    fn example1() {
        let w1= Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2= Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83");
        
        let mut grid = Grid::new();
        grid.add_wire(&w1);
        grid.add_wire(&w2);

        println!("intersections: {:?}", grid.intersections);

        let nearest = grid.get_nearest_intersection();
        println!("nearest: {:?}", nearest);

        assert_eq!(nearest.distance(), 159);
    }
    #[test]
    fn example2() {
        let w1= Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2= Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        
        let mut grid = Grid::new();
        grid.add_wire(&w1);
        grid.add_wire(&w2);

        let nearest = grid.get_nearest_intersection();
        assert_eq!(nearest.distance(), 135);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn get_direction(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => { panic!("Unexpected direction {}", c)}
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Segment {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Wire {
    segments: Vec<Segment>
}


impl Segment {
    fn parse(s: &str) -> Self {
        let dir = get_direction(s.chars().next().unwrap());
        let dist : u32 = s[1..].parse::<u32>().unwrap();
        Segment {
            direction: dir,
            distance: dist,
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x:x, y:y}
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Wire {
    fn parse(s: &str) -> Self {
        let v = s.split(',').map(|x| Segment::parse(x)).collect();

        Wire {
            segments: v
        }
    }
}
use std::collections::{HashSet, HashMap};


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct WireCellState<'a> {
    wire: &'a Wire,
    steps: u32,
}

impl<'a> WireCellState<'a> {
    fn new(wire: &'a Wire,
    steps: u32) -> Self {
        Self {
            wire: wire,
            steps: steps
        }
    }
}

struct GridCell<'a> {
    wires: HashMap<&'a Wire, u32>
}

impl<'a> GridCell<'a> {
    fn new() -> Self {
        GridCell { wires: HashMap::new() }
    }
}

struct Grid<'a> {
    cells: HashMap<Point, GridCell<'a>>,
    //center: Point,

    intersections: HashSet<Point>,
}

use std::convert::TryInto;

impl<'a> Grid<'a> {
    pub fn new() -> Self {

        Grid {
            cells: HashMap::new(),
            //center: Point::new((COLS / 2).try_into().unwrap(), (ROWS / 2).try_into().unwrap()),
            intersections: HashSet::new(),
        }
    }


    

    fn get_cell(&self, point: &Point) -> &GridCell<'a> {
        //let reference_location = index.add(&self.center);

        self.cells.get(point).unwrap()
        //let mut cell = &mut self.cells[reference_location.y as usize][reference_location.x as usize];

    }

    fn get_cell_mut(&mut self, point: &Point) -> &mut GridCell<'a> {
        //let reference_location = index.add(&self.center);

        if !self.cells.contains_key(point) {
            self.cells.insert(*point, GridCell::new());
        }
        
        self.cells.get_mut(point).unwrap()
        //let mut cell = &mut self.cells[reference_location.y as usize][reference_location.x as usize];

    }
    fn add_wire(&mut self, wire :&'a Wire) {
        let mut index = Point::new(0, 0);

        let mut steps = 0;
        for segment in &wire.segments {
            for i in 0..segment.distance {
                match segment.direction {
                    Direction::Up => {
                        index.y += 1;
                    },
                    Direction::Down => {
                        index.y -= 1;
                    },
                    Direction::Left => {
                        index.x -= 1;
                    },
                    Direction::Right => {
                        index.x += 1;
                    }
                }
                let mut cell = self.get_cell_mut(&index);

                steps += 1;

                cell.wires.insert(wire, steps);
                if cell.wires.len() > 1 {
                    self.intersections.insert(index);
                }
            }
        }
       
    }

    fn get_total_steps(&self, point: &Point) -> u32 {
        let cell = self.get_cell(point);

        let sum = cell.wires.values().sum();

        sum
    }
    fn get_nearest_intersection(&self) -> Point {
        let mut col: Vec<&Point> = self.intersections.iter().collect();
        col.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());
        **col.iter().next().unwrap()
    }

    fn get_cheapest_intersection(&self) -> Point {
        let mut col: Vec<&Point> = self.intersections.iter().collect();
        col.sort_by(|a, b| self.get_total_steps(a).partial_cmp(&self.get_total_steps(b)).unwrap());
        **col.iter().next().unwrap()
        
    }
}


use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    
    println!("Hello, world!");

    let mut file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    
    let mut wires : Vec<Wire> = Vec::new();
    
    
    for line in buf_reader.lines() {
        let l = line.unwrap();

        let w = Wire::parse(&l);
        println!("Wire:  {:?}", w);
        wires.push(w);
    }

    let mut grid = Grid::new();
    
    println!("Grid ready");
    for wire in &wires {

        grid.add_wire(&wire);
        
        println!("Wire added");
    }

    println!("intersections: {:?}", grid.intersections);

    let nearest = grid.get_nearest_intersection();
    println!("nearest: {:?} ({})", nearest, nearest.distance());
    let cheapest = grid.get_cheapest_intersection();
    println!("cheapest: {:?} ({})", cheapest, grid.get_total_steps(&cheapest));
}
