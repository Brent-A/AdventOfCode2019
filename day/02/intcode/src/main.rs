#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let mut program = [1,0,0,0,99];
        execute(&mut program);
        assert_eq!([2,0,0,0,99], program);
    }

    #[test]
    fn example2() {
        let mut program = [2,3,0,3,99];
        execute(&mut program);
        assert_eq!([2,3,0,6,99], program);
    }

    #[test]
    fn example3() {
        let mut program = [2,4,4,5,99,0];
        execute(&mut program);
        assert_eq!([2,4,4,5,99,9801], program);
    }

    #[test]
    fn example4() {
        let mut program = [1,1,1,4,99,5,6,0,99];
        execute(&mut program);
        assert_eq!([30,1,1,4,2,5,6,0,99], program);
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::convert::TryInto;

fn execute(program: &mut [u32]) {
    let mut pc = 0;

    loop {
        let opcode = program[pc];

        if opcode == 99 {
            break;
        }

        let (idx1, idx2) = (program[pc + 1] as usize, program[pc + 2] as usize);
                let outputidx = program[pc + 3] as usize;
        match opcode {
            1 => {
                program[outputidx] = program[idx1] + program[idx2];       
            }
            2 => {
                program[outputidx] = program[idx1] * program[idx2];
            }
            _ => { panic!("Unexpected opcode {}", opcode); }
        }
        pc += 4;
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    //let mut file = File::open("input.txt").unwrap().read_to_string();
    //let mut buf_reader = BufReader::new(file);

    //let program = buf_reader.read_to_string().unwrap()
    //   .split(",");

    let search = 19690720;

    let original : Vec<u32> = file.split(",").map(|x| x.parse::<u32>().unwrap()).collect();


    for noun in 0..original.len() {
        for verb in 0..original.len() {
            let mut program = original.clone();

            
            program[1] = noun.try_into().unwrap();
            program[2] = verb.try_into().unwrap();

            execute(&mut program);

            let output = program[0];

            if output == search {
                println!("noun: {} verb: {} output: {}", noun, verb, output);
                println!("result: {}", 100 * noun + verb);
            }
        }
    }
}
