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

    fn reduce_slope(&self) -> Self {
        let a = max(self.x.abs(), self.y.abs());
        for i in (1..(a + 1)).rev() {
            if self.x % i == 0 && self.y % i == 0 {
                return Self {
                    x: self.x / i,
                    y: self.y / i,
                };
            }
        }
        *self
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

fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

#[test]
fn reduce_slope() {
    assert_eq!(Point::new(1, 1), Point::new(1, 1).reduce_slope());
    assert_eq!(Point::new(1, 2), Point::new(4, 8).reduce_slope());
    assert_eq!(Point::new(1, -1), Point::new(10, -10).reduce_slope());
    assert_eq!(Point::new(2, 3), Point::new(6, 9).reduce_slope());
    assert_eq!(Point::new(1, 0), Point::new(5, 0).reduce_slope());
}

#[derive(Debug)]
struct Astroid {
    location: Point,
    can_see: HashSet<Point>,
}

impl Astroid {
    fn new(location: Point) -> Self {
        Self {
            location: location,
            can_see: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct AstroidMap {
    map: HashMap<Point, Astroid>,
    max_point: Point,
}

impl AstroidMap {
    fn parse(string: &str) -> Self {
        let mut map = HashMap::new();
        let mut y = 0;
        let mut max = Point::new(0, 0);
        for line in string.lines() {
            let line = line.trim();
            let mut x = 0;
            for c in line.chars() {
                if c == '#' || c == 'X' {
                    let p = Point::new(x, y);
                    map.insert(p, Astroid::new(p));

                    if p.x > max.x {
                        max = Point::new(p.x, max.y);
                    }

                    if p.y > max.y {
                        max = Point::new(max.x, p.y);
                    }
                }
                x += 1;
            }
            y += 1;
        }

        let mut m = AstroidMap {
            map: map,
            max_point: max,
        };
        m.compute_seen();
        m
    }

    fn most_seen(&self) -> Option<&Astroid> {
        let mut max: Option<&Astroid> = None;
        for a in self.map.values() {
            if let Some(m) = max {
                if a.can_see.len() > m.can_see.len() {
                max = Some(a);
                }
            } else {
                max = Some(a);
            }
        }
        max
    }

    fn laser_sequence(&mut self, base: Point) -> Vec<Point> {

        let mut shots : Vec<Point> = Vec::new();

        while self.map.len() > 1 {
            println!("{} astroids", self.map.len());
            let mut targets :Vec<Point> = self.map.get(&base).unwrap().can_see.iter().map(|x| *x).collect();
            targets.sort_by(|a, b| { 
                let angle1 = (*a - base).angle();
                let angle2 = (*b - base).angle();
                
                angle1.partial_cmp(&angle2).unwrap() });
            
            println!("shooting: {:?}", targets);
            for t in &targets {
                self.map.remove(&t);
            }

            shots.append(&mut targets);


            self.compute_seen();
        }

        shots
    }

    fn compute_seen(&mut self) {
        for a in self.map.values_mut() {
            a.can_see.clear();
        }

        let points: Vec<Point> = self.map.keys().map(|x| *x).collect();

        for from in &points {
            for to in &points {
                if from == to {
                    continue;
                }
                let offset = to - from;
                let slope = offset.reduce_slope();

                let mut test_point = *from + slope;
                let mut occluded = false;
                while test_point.x <= self.max_point.x
                    && test_point.y <= self.max_point.y
                    && test_point.x >= 0
                    && test_point.y >= 0
                    && test_point != *to
                {
                    if self.map.contains_key(&test_point) {
                        occluded = true;
                        //println!("{:?} to {:?} occluded by {:?}", from, to , test_point);
                        break;
                    }
                    test_point = test_point + slope;
                }
                if !occluded {
                    self.map.get_mut(&from).unwrap().can_see.insert(*to);
                }
            }
        }
    }
}


#[test]
fn AstroidMapTest1() {
    println!("");
    let mut map = AstroidMap::parse(
        ".#..#
    .....
    #####
    ....#
    ...##",
    );

    map.compute_seen();

    //println!("map: {:?}", map);
    let most = map.most_seen();
    println!("most: {:?}", most);
}

#[test]
fn LaserTest() {
    let mut map = AstroidMap::parse(".#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....X...###..
    ..#.#.....#....##");

    let base = Point::new(8, 3);

    let sequence = map.laser_sequence(base);

    println!("SEQUENCE: {:?}", sequence);
}

#[test]
fn AstroidMapTest2() {
    println!("");
    let mut map = AstroidMap::parse(
        ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##",
    );

    //println!("map: {:?}", map);
    let most = map.most_seen();

    assert_eq!(most.unwrap().location, Point::new(11,13));
    assert_eq!(most.unwrap().can_see.len(), 210);

    let laser_shots = map.laser_sequence(most.unwrap().location);

    assert_eq!(laser_shots[199], Point::new(8, 2));
}

fn main() {
    let mut map = AstroidMap::parse(
        "..#..###....#####....###........#
        .##.##...#.#.......#......##....#
        #..#..##.#..###...##....#......##
        ..####...#..##...####.#.......#.#
        ...#.#.....##...#.####.#.###.#..#
        #..#..##.#.#.####.#.###.#.##.....
        #.##...##.....##.#......#.....##.
        .#..##.##.#..#....#...#...#...##.
        .#..#.....###.#..##.###.##.......
        .##...#..#####.#.#......####.....
        ..##.#.#.#.###..#...#.#..##.#....
        .....#....#....##.####....#......
        .#..##.#.........#..#......###..#
        #.##....#.#..#.#....#.###...#....
        .##...##..#.#.#...###..#.#.#..###
        .#..##..##...##...#.#.#...#..#.#.
        .#..#..##.##...###.##.#......#...
        ...#.....###.....#....#..#....#..
        .#...###..#......#.##.#...#.####.
        ....#.##...##.#...#........#.#...
        ..#.##....#..#.......##.##.....#.
        .#.#....###.#.#.#.#.#............
        #....####.##....#..###.##.#.#..#.
        ......##....#.#.#...#...#..#.....
        ...#.#..####.##.#.........###..##
        .......#....#.##.......#.#.###...
        ...#..#.#.........#...###......#.
        .#.##.#.#.#.#........#.#.##..#...
        .......#.##.#...........#..#.#...
        .####....##..#..##.#.##.##..##...
        .#.#..###.#..#...#....#.###.#..#.
        ............#...#...#.......#.#..
        .........###.#.....#..##..#.##...",
    );

    map.compute_seen();

    //println!("map: {:?}", map);
    let most = map.most_seen();
    println!("most: {:?}", most.unwrap().can_see.len());

    
    let laser_shots = map.laser_sequence(most.unwrap().location);

    println!("200th: {:?}", laser_shots[199]);
}
