use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn mv(&self, direction: &Direction) -> Self {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MapTile {
    Open,
    Wall,
    Portal(String, Point),
}

impl MapTile {
    fn from_ascii(c: char) -> Option<MapTile> {
        match c {
            '.' => Some(MapTile::Open),
            '#' => Some(MapTile::Wall),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            MapTile::Open => &"  ",
            MapTile::Wall => &"##",
            MapTile::Portal(label, _) => &label, 
        }
    }

    fn traversable(&self) -> bool {
        match self {
            MapTile::Open => true,
            MapTile::Wall => false,
            MapTile::Portal(_, _) => true,
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub tiles: HashMap<Point, MapTile>,
    pub labeled: HashMap<String, Point>,
}

impl Map {
    pub fn new(chars: &str) -> Self {
        let mut s = Self {
            tiles: HashMap::new(),
            labeled: HashMap::new(),
        };

        let lines: Vec<&str> = chars.lines().collect();

        let mut p;
        for (y, line) in lines.iter().enumerate() {
            //let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                p = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
                if let Some(tile) = MapTile::from_ascii(c) {
                    if tile == MapTile::Open {
                        for direction in &DIRECTIONS {
                            let l1 = p.mv(&direction);
                            if let Some(c1) = lines[l1.y as usize].chars().nth(l1.x as usize) {
                                let l2 = l1.mv(&direction);
                                if let Some(c2) = lines[l2.y as usize].chars().nth(l2.x as usize) {
                                    if c1.is_ascii_uppercase() {
                                        let label: String = match direction {
                                            Direction::North => format!("{}{}", c2, c1),
                                            Direction::East => format!("{}{}", c1, c2),
                                            Direction::West => format!("{}{}", c2, c1),
                                            Direction::South => format!("{}{}", c1, c2),
                                        };
                                        //println!("found {} facing {:?} at {:?} ({:?},{:?})", label, direction, p, l1, l2);
                                        //println!("{}", line);
                                        //println!("{}", lines[l1.y as usize]);
                                        //println!("{}", lines[l2.y as usize]);
                                        s.labeled.insert(label.clone(), p);
                                        s.tiles.insert(l1, MapTile::Portal(label, p));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    s.tiles.insert(p, tile);
                }
            }
        }
        s
    }

    //fn intersections(&self) -> Vec<Point> {
    //self.tiles.iter().filter(|(p, t)| t.traversable() && self.traversable_neighbors(p).len() == 4).map(|(p,_)| *p).collect()
    //}
    pub fn get_tile(&self, point: &Point) -> &MapTile {
        let x = self.tiles.get(point);
        if x.is_none() {
            println!("get_tile({:?})", point);
        }
        x.unwrap()
    }

    pub fn neighbors(&self, point: &Point) -> Vec<(Point, &MapTile)> {
        let mut v = Vec::new();
        for d in &DIRECTIONS {
            let p2 = d.move_point(&point);
            let tile = self.get_tile(&p2);
            if let MapTile::Portal(label, point) = tile {
                if let Some((otherpoint, otherlabel, ap)) = self.portals().iter().filter(|(p, l, ap)| *l == label && *p != p2).next() {
                    v.push((**ap, self.get_tile(&otherpoint)));
                }
                else {
                    println!("Unmatched label: {}", label);
                    assert!(label == "AA" || label == "ZZ");
                }
            }
            else {
                v.push((p2, tile));
            }
        }
        v
    }

    pub fn portals(&self) -> Vec<(Point, &String, &Point)> {
        self.tiles
            .iter()
            .filter_map(|(p, t)| match t {
                MapTile::Portal(l, ap) => Some((*p, l, ap)),
                _ => None,
            })
            .collect()
    }

    pub fn traversable_neighbors(&self, point: &Point) -> Vec<(Point, &MapTile)> {
        self.neighbors(point)
            .iter()
            .filter(|x| x.1.traversable())
            .map(|x| *x)
            .collect()
    }

    pub fn get_cost_map(&self, point: &Point) -> HashMap<Point, i32> {
        let mut cost_map = HashMap::new();
        let mut search: Vec<(Point, i32)> = Vec::new();
        search.push((*point, 0));
        cost_map.insert(*point, 0);

        while search.len() > 0 {
            //println!("seachlen: {}", search.len());
            let (point, cost) = search.remove(0);

            for (p, t) in self.traversable_neighbors(&point) {
                if *cost_map.get(&p).unwrap_or(&10000) > cost {
                    //println!("{},{} = {}   {}", p.x, p.y, cost, cost_map.len());
                    cost_map.insert(p, cost);
                    search.push((p, cost + 1));
                }
            }
        }
        cost_map
    }

    /*
    fn find_path(&self, start: &Point, end: &Point) -> Option<(i32, Vec<char>, Vec<char>)> {
        let mut cost_map = self.get_cost_map(end, false);

        if cost_map.get(start).is_some() {
            //println!(" calculated {} points", cost_map.len());
            //println!(" finding path");
            let mut doors = Vec::new();
            let mut keys = Vec::new();
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

                let next_tile = self.tiles.get(&p);
                if next_tile.is_none() {
                    println!("Start: {:?} End: {:?}", start, end);
                    println!("No tile at {:?}", p);
                    println!("options: {:?}", options);

                    println!("Cost_map(start): {:?}", cost_map.get(start));
                }
                if let MapTile::Door(d) = self.tiles.get(&p).expect("No tile") {
                    doors.push(*d);
                }
                if let MapTile::Key(k) = self.tiles.get(&p).expect("No tile") {
                    keys.push(*k);
                }
            }
            Some((*cost_map.get(start).unwrap(), doors, keys))
        } else {
            None
        }
    }*/

    fn draw(&self) {
        let mut upper_left = Point::new(0, 0);
        let mut lower_right = Point::new(0, 0);

        for point in self.tiles.keys() {
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
                let tile = self.get_tile(&Point::new(x, y));
                print!("{}", tile.to_string());
            }
            println!("");
        }
    }
}
