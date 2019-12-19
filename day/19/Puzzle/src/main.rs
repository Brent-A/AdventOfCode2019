use intcode::*;

fn hit(program: &[Integer], x: Integer, y: Integer) -> bool {
    let mut m = Machine::new(&program);
    m.input().send(Value(x));
    m.input().send(Value(y));

    m.run();

    let v = m.output().as_ref().unwrap().recv().unwrap();

    v.0 != 0
}
fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program: Vec<Integer> = file
        .split(",")
        .map(|x| x.parse::<Integer>().unwrap())
        .collect();

    let mut ystart = 0;
    let mut c = 0;
    for x in 100..100000 {
        let mut iny = false;
        for y in ystart..10000 {
            if hit(&program, x, y) {
                if iny == false {
                    ystart = y;
                    println!("{}, {}", x, y);
                }
                iny = true;
                if hit(&program, x - 100, y)
                    && hit(&program, x - 100, y + 100)
                    && hit(&program, x, y + 100)
                {
                    println!("fits {}, {}", x-100, y);
                    println!("{}", x * 10000 + y);
                }
            } else {
                break;
            }
        }
    }
    println!("");
}
