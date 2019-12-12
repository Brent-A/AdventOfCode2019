
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

    fn axis(&self, axis: &Axis) -> &T {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }

    fn axis_mut(&mut self, axis: &Axis) -> &mut T {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
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

impl Axis {
    const VALUES: [Axis;3] = [Axis::X, Axis::Y, Axis::Z]; 

    fn values() -> &'static [Axis] {
        &Axis::VALUES
    }
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

    fn same(&self, other: &Self, axis: &Axis) -> bool {

        for i in 0..self.moons.len() {
            if self.moons[i].position.axis(axis) != other.moons[i].position.axis(axis) {
                return false;
            }
            if self.moons[i].velocity.axis(axis) != other.moons[i].velocity.axis(axis) {
                return false;
            }
        }

        return true;
    }

    fn step(&self) -> Self {
        let mut moons = self.moons.clone();
        for a in 0..moons.len() {
            for b in 0..moons.len() {
                let m1 = &mut moons[a];
                let m2 = &self.moons[b];

                for axis in Axis::values() {
                    if m1.position.axis(axis) < m2.position.axis(axis) {
                        *m1.velocity.axis_mut(axis) += 1;
                    }
                    else if m1.position.axis(axis) > m2.position.axis(axis) {
                        *m1.velocity.axis_mut(axis) -= 1;
                    }
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

    let mut periods = Vec::new();
    for axis in Axis::values() {
        println!("Axis: {:?}", axis);
        let mut next = system.clone();
        loop {
            next = next.step();
            if next.same(&system, axis) {
                println!("Same: {:?} -> {:?}", system.steps, next.steps);
                periods.push(next.steps);
                break;
            }
        }
    }

    let lcm1 = num::integer::lcm(periods[0], periods[1]);
    let lcm2 = num::integer::lcm(lcm1, periods[2]);

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
