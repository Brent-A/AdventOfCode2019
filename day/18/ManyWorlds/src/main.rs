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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Open,
    Wall,
    Key(char),
    Door(char),
    Entrance,
}

type Keychain = Vec<char>;

impl MapTile {
    fn from_ascii(c: char) -> MapTile {
        match c {
            '.' => MapTile::Open,
            '#' => MapTile::Wall,
            '@' => MapTile::Entrance,
            c => {
                if c.is_lowercase() {
                    MapTile::Key(c.to_ascii_uppercase())
                } else {
                    MapTile::Door(c)
                }
            }
        }
    }

    fn to_char(&self) -> char {
        match self {
            MapTile::Open => ' ',
            MapTile::Wall => '#',
            MapTile::Key(c) => c.to_ascii_lowercase(),
            MapTile::Door(c) => *c,
            MapTile::Entrance => '@',
        }
    }

    fn traversable(&self, keys: &Keychain, lock_doors: bool) -> bool {
        match self {
            MapTile::Open => true,
            MapTile::Key(_) => true,
            MapTile::Wall => false,
            MapTile::Entrance => true,
            MapTile::Door(x) => keys.contains(&x) || !lock_doors,
        }
    }
}

#[derive(Clone)]
struct Map {
    tiles: HashMap<Point, MapTile>,
    location: Point,
    keychain: Keychain,
    traveled: i32,
}

#[derive(Debug, Clone)]
struct Path {
    origin: char,
    dest: char,
    doors: Vec<char>,
    pathkeys: Vec<char>,
    distance: i32,
}

struct Branch {
    children: Vec<char>,
}

impl Map {
    fn new(chars: &str) -> Self {
        let mut s = Self {
            tiles: HashMap::new(),
            location: Point::new(0, 0),
            keychain: Keychain::new(),
            traveled: 0,
        };

        //println!("chars: {:?}", chars);
        let mut p = Point::new(0, 0);
        for (y, line) in chars.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                p = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
                let tile = MapTile::from_ascii(c);
                s.tiles.insert(p, tile);

                if tile == MapTile::Entrance {
                    s.location = p;
                }
            }
        }
        s
    }

    fn get_paths(&self) -> HashMap<char, Vec<Path>> {
        let keys = self.keys();
        let mut paths = HashMap::new();

        let mut origin = Vec::new();
        for a in 0..keys.len() {
            let mut v = Vec::new();
            for b in 0..keys.len() {
                if a == b {
                    continue;
                }
                if let Some((cost, doors, pathkeys)) = self.find_path(&keys[a].0, &keys[b].0) {
                    v.push(Path {
                        origin: keys[a].1,
                        dest: keys[b].1,
                        doors: doors,
                        distance: cost + 1,
                        pathkeys: pathkeys,
                    });
                }
            }
            if let Some((cost, doors, pathkeys)) = self.find_path(&self.location, &keys[a].0) {
                origin.push(Path {
                    origin: '@',
                    dest: keys[a].1,
                    doors: doors,
                    distance: cost + 1,
                    pathkeys: pathkeys,
                });
            }

            paths.insert(keys[a].1, v);
        }
        paths.insert('@', origin);

        paths
    }

    //fn intersections(&self) -> Vec<Point> {
    //self.tiles.iter().filter(|(p, t)| t.traversable() && self.traversable_neighbors(p).len() == 4).map(|(p,_)| *p).collect()
    //}
    fn get_tile(&self, point: &Point) -> MapTile {
        *self.tiles.get(point).unwrap_or(&MapTile::Open)
    }

    fn neighbors(&self, point: &Point) -> Vec<(Point, MapTile)> {
        let mut v = Vec::new();
        for d in &DIRECTIONS {
            let p2 = d.move_point(&point);
            v.push((p2, self.get_tile(&p2)));
        }
        v
    }

    fn traversable_neighbors(&self, point: &Point, lock_doors: bool) -> Vec<(Point, MapTile)> {
        self.neighbors(point)
            .iter()
            .filter(|x| x.1.traversable(&self.keychain, lock_doors))
            .map(|x| *x)
            .collect()
    }

    fn accessible_keys(&self) -> Vec<(Point, char, i32)> {
        let mut cost_map = self.get_cost_map(&self.location, true);

        self.keys()
            .iter()
            .filter(|(p, t)| cost_map.contains_key(p))
            .map(|(p, t)| (*p, *t, *cost_map.get(p).unwrap()))
            .collect()
    }

    fn collect_key(&self, key: (Point, char, i32)) -> Self {
        let mut next = self.clone();

        next.traveled += key.2;
        next.tiles.insert(key.0, MapTile::Open);
        next.keychain.push(key.1);
        next.location = key.0;

        //println!("next: keychain={:?}", next.keychain);
        next
    }

    fn keys(&self) -> Vec<(Point, char)> {
        self.tiles
            .iter()
            .filter(|(p, t)| match t {
                MapTile::Key(_) => true,
                _ => false,
            })
            .map(|(p, t)| {
                if let MapTile::Key(k) = t {
                    (*p, *k)
                } else {
                    panic!("not a key");
                }
            })
            .collect()
    }

    fn doors(&self) -> Vec<(Point, char)> {
        self.tiles
            .iter()
            .filter(|(p, t)| match t {
                MapTile::Door(_) => true,
                _ => false,
            })
            .map(|(p, t)| {
                if let MapTile::Door(k) = t {
                    (*p, *k)
                } else {
                    panic!("not a key");
                }
            })
            .collect()
    }

    fn get_cost_map(&self, point: &Point, lock_doors: bool) -> HashMap<Point, i32> {
        let mut cost_map = HashMap::new();
        let mut search: Vec<(Point, i32)> = Vec::new();
        search.push((*point, 0));
        cost_map.insert(*point, 0);

        while search.len() > 0 {
            let (point, cost) = search.remove(0);

            for (p, t) in self.traversable_neighbors(&point, lock_doors) {
                if *cost_map.get(&p).unwrap_or(&10000) > cost {
                    cost_map.insert(p, cost);
                    match t {
                        MapTile::Key(_) => {
                            if !lock_doors {
                                search.push((p, cost + 1));
                            }
                        }
                        _ => {
                            search.push((p, cost + 1));
                        }
                    }
                }
            }
        }
        cost_map
    }

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
    }

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
                print!("{}", tile.to_char());
            }
            println!("");
        }
    }
}

/*
fn shortest_path(map: &Map) -> Option<(Point, MapTile, i32)> {
    let mut next: Option<(Point, MapTile, i32)> = None;
    for key in map.accessible_keys() {
        let newmap = map.collect_key(key);
        let newpoint = shortest_path(&newmap);

        if let Some((np, nt, nc)) = newpoint {
            if next.is_none() {
                next = newpoint;
            } else if let Some((p, t, c)) = next {
                if nc < c {
                    next = newpoint;
                }
            }
        }
    }

    if next.is_some() {
        println!("{:?}", next);
    } else {
        println!("Found all keys in {}. {:?}", map.traveled, map.keychain);
        //map.draw();
    }
    next
}
*/

#[derive(Hash, Eq, PartialEq, Debug)]
struct StateKey {
    heldkeys: String,
    location: char,
}

impl StateKey {
    fn new(path: &Vec<char>) -> StateKey {
        let mut pathsorted = path.clone();
        pathsorted.sort();
        Self {
            heldkeys: pathsorted.iter().collect(),
            location: *path.last().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
struct FullPath {
    path: Vec<char>,
    length: i32,
}

impl FullPath {
    fn next_steps(&self, possible_paths: &HashMap<char, Vec<Path>>) -> Vec<FullPath> {
        let mut v = Vec::new();
        let location = self.path.last().unwrap();
        for try_path in possible_paths.get(&location).unwrap().iter() {
            if !self.path.contains(&try_path.dest)
                && try_path.doors.iter().all(|x| self.path.contains(x))
            {
                let mut newpath = self.path.clone();
                for p in &try_path.pathkeys {
                    if !newpath.contains(p) {
                        newpath.push(*p);
                    }
                }

                v.push(Self {
                    path: newpath,
                    length: self.length + try_path.distance,
                });
            }
        }
        v
    }

    fn state_key(&self) -> StateKey {
        StateKey::new(&self.path)
    }
}

type ShortPathMap = HashMap<StateKey, FullPath>;

fn print_spm(shortest_path_to_state: &ShortPathMap) {
    let mut max_depth = 0;
    let mut o = Vec::new();
    for (k, v) in shortest_path_to_state.iter() {
        o.push((
            k.heldkeys.clone() + "+" + &k.location.to_string(),
            v.clone(),
        ));
        if v.path.len() > max_depth {
            max_depth = v.path.len();
        }
    }
    o.sort_by(|a, b| {
        let c = a.0.len().partial_cmp(&b.0.len()).unwrap();
        if c == std::cmp::Ordering::Equal {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            c
        }
    });
    for (k, v) in o {
        println!("{:?}: {:?}", k, v);
    }
    println!("Solved to depth: {}", max_depth);
}

fn short_path(possible_paths: &HashMap<char, Vec<Path>>) -> ShortPathMap {
    let mut position = '@';
    let mut shortest_path_to_state = ShortPathMap::new();

    let mut paths_to_search: Vec<FullPath> = Vec::new();

    paths_to_search.push(FullPath {
        path: vec!['@'],
        length: 0,
    });

    let mut last_print = std::time::Instant::now();

    while (paths_to_search.len() > 0) {
        let search = paths_to_search.remove(0);

        for next in search.next_steps(possible_paths) {
            let key = next.state_key();
            if let Some(path) = shortest_path_to_state.get(&key) {
                if path.length > next.length {
                    shortest_path_to_state.insert(key, next.clone());
                    paths_to_search.push(next);
                }
            } else {
                shortest_path_to_state.insert(key, next.clone());
                paths_to_search.push(next);
            }
        }

        if last_print.elapsed() > std::time::Duration::from_secs(10) {
            println!("");
            println!("");

            print_spm(&shortest_path_to_state);

            last_print = std::time::Instant::now();
        }
    }

    print_spm(&shortest_path_to_state);

    shortest_path_to_state
}

/*
fn shortest_path(possible_paths: &HashMap<char, Vec<Path>>, path: &Vec<char>, cost: i32) -> (i32, Vec<char>) {


    let position = path.last().unwrap();

    let mut shortest : Option<(i32, Vec<char>)> = None;
    for try_path in possible_paths.get(&position).unwrap().iter() {

        if !path.contains(&try_path.dest) && try_path.doors.iter().all(|x| path.contains(x)) {

            if shortest.is_none() {
                let mut attempt = path.clone();
                for pathkey in &try_path.pathkeys {
                    if !attempt.contains(pathkey) {
                        attempt.push(*pathkey);
                    }
                }
                let attempt_distance = shortest_path(possible_paths, &attempt, cost + try_path.distance);

                shortest = Some(attempt_distance);
            } else {
                if let Some(shortest_value) = shortest.take() {

                    let shortest_added_cost = shortest_value.0 - cost;

                    let mut attempt = path.clone();
                    for pathkey in &try_path.pathkeys {
                        if !attempt.contains(pathkey) {
                            attempt.push(*pathkey);
                        }
                    }
                    let attempt_distance = shortest_path(possible_paths, &attempt, cost + try_path.distance);



                    if shortest_value.0 < attempt_distance.0 {
                        shortest = Some(shortest_value);
                    }
                    else {
                        shortest = Some(attempt_distance);
                    }
                }
            }

        }
    }

    if shortest.is_none() {
        println!("found possibility: {}, {:?}", cost, path);
    }


    shortest.unwrap_or((cost, path.clone()))
}
*/

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    //let file = "#################
    //#i.G..c...e..H.p#
    //########.########
    //#j.A..b...f..D.o#
    //########@########
    //#k.E..a...g..B.n#
    //########.########
    //#l.F..d...h..C.m#
    //#################";

    //let file = "########################
    //#...............b.C.D.f#
    //#.######################
    //#.....@.a.B.c.d.A.e.F.g#
    //########################";

    let map = Map::new(&file);

    map.draw();

    println!("keys ({}): {:?}", map.keys().len(), map.keys());
    println!("doors ({}): {:?}", map.doors().len(), map.doors());

    println!("reachable keys: {:?}", map.accessible_keys());

    let mut possible_paths = map.get_paths();
    /*
        let mut optimized_paths = possible_paths.clone();

        let keys : Vec<char> = map.keys().iter().map(|(p, k)| *k).collect();

        let mut reduced = true;
        while (reduced) {
            reduced = false;
            for (k, outpaths) in possible_paths.iter() {

                for outkey in keys {
                    // All paths to k cross through outkey
                    if outpaths.iter().all(|p| p.pathkeys[0] == k {

                    }
                }

            }
        }
    */
    println!("possible_path: {:?}", possible_paths);
    let sp = short_path(&possible_paths);

    println!("path: {:?}", sp);

    // 3868 is too high
}
