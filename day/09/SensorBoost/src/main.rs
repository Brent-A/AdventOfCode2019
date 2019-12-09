use intcode::*;

#[test]
fn testfn() {
    let program = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];

    let mut machine = Machine::new(&program);
    
    println!("Result is: {:?}", machine.run());
    let output : Vec<Value> = machine.output().as_ref().unwrap().try_iter().collect();
    println!("Output is: {:?}", output);
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let original: Vec<Integer> = file.split(",").map(|x| x.parse::<Integer>().unwrap()).collect();

    let mut machine = Machine::new(&original);
    machine.input().send(Value(1)).unwrap();

    let result = machine.run();
    println!("Output is: {:?}", result);

    let output : Vec<Value> = machine.output().as_ref().unwrap().try_iter().collect();
    println!("Output is: {:?}", output);

}
