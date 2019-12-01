#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples1() {
        assert_eq!(2, fuel(12));
        assert_eq!(2, fuel(14));
        assert_eq!(654, fuel(1969));
        assert_eq!(33583, fuel(100756));
    }

    #[test]
    fn examples2() {
        assert_eq!(2, total_fuel(14));
        assert_eq!(966, total_fuel(1969));
        assert_eq!(50346, total_fuel(100756));
    }

    #[test]
    fn edge_cases() {
        assert_eq!(0, fuel(0));
    }
}


fn fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel(component_mass: u32) -> u32 {
    let mut mass = component_mass;
    let mut f = fuel(mass);
    let mut total_fuel = f;
    loop {
        f = fuel(f);
        total_fuel += f;
        if f == 0 {
            return total_fuel;
        }
    }
}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut sum = 0;
    let mut total_sum = 0;
    for line in buf_reader.lines() {
        let l = line.unwrap();
        let v = l.parse::<u32>().unwrap();
        let r = fuel(v);
        let t = total_fuel(v);
        println!("{} -> {}, {}", v, r, t);
        sum += r;
        total_sum += t;
    }

    println!("Sum: {}", sum);
    println!("Total Sum: {}", total_sum);
}
