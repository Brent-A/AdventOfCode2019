
type Unit = i32;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

use std::default::Default;

impl<T> Vector3<T> {
    fn zero() -> Self where T : Default {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }

    fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl<T> std::ops::Add for Vector3<T> where T : std::ops::Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Moon {
    position: Vector3<Unit>,
    velocity: Vector3<Unit>,
}

impl Moon {
    fn new(position: &Vector3<Unit>) -> Self {
        Self {
            position: *position,
            velocity: Vector3::zero(),
        }
    }

    fn potential(&self) -> Unit {
        self.position.x.abs() +
        self.position.y.abs() +
        self.position.z.abs()
    }

    fn kinetic(&self) -> Unit {
        self.velocity.x.abs() +
        self.velocity.y.abs() +
        self.velocity.z.abs()
    }

    fn total_energy(&self) -> Unit {
        self.potential() * self.kinetic()
    }
}


#[derive(Debug, Clone)]
struct System {
    moons: Vec<Moon>,
    steps: usize
}

impl System {
    fn new(moons: &[Moon]) -> Self {
        System {
            moons: moons.to_vec(),
            steps: 0,
        }
    }

    fn same(&self, other: &Self, axis: Axis) -> bool {

        match axis {
            Axis::X => {
                for i in 0..self.moons.len() {
                    if self.moons[i].position.x != other.moons[i].position.x {
                        return false;
                    }
                    if self.moons[i].velocity.x != other.moons[i].velocity.x {
                        return false;
                    }
                }
            },
            Axis::Y => {
                for i in 0..self.moons.len() {
                    if self.moons[i].position.y != other.moons[i].position.y {
                        return false;
                    }
                    if self.moons[i].velocity.y != other.moons[i].velocity.y {
                        return false;
                    }
                }
            },
            Axis::Z => {
                for i in 0..self.moons.len() {
                    if self.moons[i].position.z != other.moons[i].position.z {
                        return false;
                    }
                    if self.moons[i].velocity.z != other.moons[i].velocity.z {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn step(&self) -> Self {
        let mut moons = self.moons.clone();
        for a in 0..moons.len() {
            for b in 0..moons.len() {
                let mut m1 = &mut moons[a];
                let m2 = &self.moons[b];

                if m1.position.x < m2.position.x {
                    m1.velocity.x += 1;
                }
                else if m1.position.x > m2.position.x {
                    m1.velocity.x -= 1;
                }

                if m1.position.y < m2.position.y {
                    m1.velocity.y += 1;
                }
                else if m1.position.y > m2.position.y {
                    m1.velocity.y -= 1;
                }

                if m1.position.z < m2.position.z {
                    m1.velocity.z += 1;
                }
                else if m1.position.z > m2.position.z {
                    m1.velocity.z -= 1;
                }

                    
            }
        }

        for m in &mut moons {
            m.position = m.position + m.velocity;
        }

        Self {
            moons: moons,
            steps: self.steps + 1
        }       
    }
}

fn main() {
    let mut system = System::new(&[
        Moon::new(&Vector3::new(3, 15, 8)),
        Moon::new(&Vector3::new(5, -1, -2)),
        Moon::new(&Vector3::new(-10, 8, 2)),
        Moon::new(&Vector3::new(8, 4, -5)),
        
    ]);
/*
    let system = System::new(&[
        Moon::new(&Vector3::new(-8, -10, 0)),
        Moon::new(&Vector3::new(5, 5, 10)),
        Moon::new(&Vector3::new(2, -7, 3)),
        Moon::new(&Vector3::new(9, -8, -3)),
        
    ]);
*/

    let xrepeat;
    let yrepeat;
    let zrepeat;
    println!("X:");
    let mut next = system.clone();
    loop {
        next = next.step();
        if next.same(&system, Axis::X) {
            println!("Same: {:?} -> {:?}", system.steps, next.steps);
            xrepeat = next.steps;
            break;
        }
    }

    println!("Y:");
    let mut next = system.clone();
    loop {
        next = next.step();
        if next.same(&system, Axis::Y) {
            println!("Same: {:?} -> {:?}", system.steps, next.steps);
            yrepeat = next.steps;
            break;
        }
    }

    println!("Z:");
    let mut next = system.clone();
    loop {
        next = next.step();
        if next.same(&system, Axis::Z) {
            println!("Same: {:?} -> {:?}", system.steps, next.steps);
            zrepeat = next.steps;
            break;
        }
    }
    let lcm1 = num::integer::lcm(xrepeat, yrepeat);
    let lcm2 = num::integer::lcm(lcm1, zrepeat);

    println!("total_period: {}", lcm2);

    /*
    println!("system: {:?}", system);

    let mut energy = 0;
    for moon in &system.moons {
        println!("{} {} {}", moon.potential(), moon.kinetic(), moon.total_energy());
        energy += moon.total_energy();
    }

    println!("energy: {}", energy);
*/
}
