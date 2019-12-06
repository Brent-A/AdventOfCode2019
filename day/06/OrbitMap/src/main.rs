mod test {
    use super::*;

    #[test]
    fn example() {
        let orbits = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

        let map = OrbitMap::parse(&orbits);

        assert_eq!(map.masses.len(), 11);

        assert_eq!(3, map.total_orbits(&"D"));

        let mut direct = 0;
        let mut indirect = 0;
        for m in map.masses.keys() {
            direct += 1;
            indirect += map.total_orbits(m);
        }

        assert_eq!(indirect, 42);
    }

    #[test]
    fn part2() {
        let orbitmap = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";

        
        let map = OrbitMap::parse(&orbitmap);

        let mut path1 = Vec::new();
        map.path(&"YOU", &mut path1);
        println!("path1: {:?}", path1);

        let mut path2 = Vec::new();
        map.path(&"SAN", &mut path2);
        println!("path2: {:?}", path2);

        assert_eq!(4, map.distance(&"YOU", &"SAN"));
    }
}

struct Mass {
    name: String,
    orbits: String,
}

use std::collections::HashMap;

struct OrbitMap {
    masses: HashMap<String, Mass>
}

impl OrbitMap {
    fn parse(text: &str) -> OrbitMap {
        let mut map = HashMap::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.trim().split(")").collect();
            let mass = Mass {
                name: parts[1].to_string(),
                orbits: parts[0].to_string(),
            };

            if map.contains_key(&mass.name) {
                panic!("Already contains {}", mass.name);
            }

            map.insert(mass.name.clone(), mass);

        }

        OrbitMap {
            masses: map
        }
    }

    fn total_orbits(&self, mass: &str) -> u32 {

        match self.masses.get(mass) {
            Option::Some(m) => {
                return self.total_orbits(&m.orbits) + 1;
            },
            Option::None => {
                return 0;
            }
        }
    }

    fn path(&self, mass: &str, path: &mut Vec<String>) {

        match self.masses.get(mass) {
            Option::Some(m) => {
                //path.push(m.name.clone());
                path.insert(0, m.name.clone());
                self.path(&m.orbits, path);
            },
            Option::None => {
                return;
            }
        }
    }

    fn distance(&self, mass1: &str, mass2: &str) -> usize {
        let mut path1 = Vec::new();
        let mut path2 = Vec::new();

        self.path(mass1, &mut path1);
        self.path(mass2, &mut path2);

        while path1[0] == path2[0] {
            path1.remove(0);
            path2.remove(0);
        }

        path1.len() + path2.len() - 2
    }
}
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

    let file = std::fs::read_to_string("input.txt").unwrap();
    
    let map = OrbitMap::parse(&file);

    let mut orbits = 0;
    for m in map.masses.keys() {

        orbits += map.total_orbits(m);
    }

    println!("orbits: {}", orbits);


    
    println!("transfers: {}", map.distance(&"YOU", &"SAN"));

}
