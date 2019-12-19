use intcode::*;

fn hit(program: &[Integer], x: Integer, y: Integer) -> bool {
    let mut m = Machine::new(&program);
    m.input().send(Value(x));
    m.input().send(Value(y));

    m.run();

    let v = m.output().as_ref().unwrap().recv().unwrap();

    v.0 != 0
}

// 012345678
//   ....
// 
// x = 5, x = 2 ()
fn fits(program: &[Integer], x: Integer, y: Integer) -> bool {
    if hit(&program, x, y)
        && hit(&program, x - 99, y)
        && hit(&program, x - 99, y + 99)
        && hit(&program, x, y + 99)
    {
        println!("fits {}, {} = {}", x - 99, y, (x-99) * 10000 + y);

        println!("UL-U: {}", hit(&program, x - 99, y - 1));
        
        println!("UL-L: {}", hit(&program, x - 100, y));
        println!("LL-D: {}", hit(&program, x - 99, y + 100));
        
        println!("LL-L: {}", hit(&program, x - 100, y + 99));
        return true;
    }
    false
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program: Vec<Integer> = file
        .split(",")
        .map(|x| x.parse::<Integer>().unwrap())
        .collect();

    let mut ystart = 0;
    let mut c = 0;
    let mut x = 10;
    let mut y = 0;

    // 0123
    loop {
        while (!hit(&program, x, y)) {
            y += 1;
        }

        let mut fit = false;
        while (fits(&program, x, y)) {
            fit = true;
            while (fits(&program, x, y)) {
                x -= 1;
            }
            x += 1;

            while (fits(&program, x, y)) {
                y -= 1;
            }
            y += 1;
        }

        if fit {
            break;
        }

        println!("{}, {}", x, y);

        x += 1;
    }
    println!("");

    // not 10880491
    // not 10750485
}
