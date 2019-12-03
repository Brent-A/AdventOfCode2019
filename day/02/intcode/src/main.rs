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

#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;

type Integer = u32;
type Memory = [Integer];

#[derive(Copy, Clone, Debug, PartialEq)]
struct Address(usize);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Value(Integer);

#[derive(Debug, PartialEq)]
enum Instruction {
    Add { arg1: Address, arg2: Address, out: Address},
    Mult { arg1: Address, arg2: Address, out: Address},
    Terminate,
}

#[derive(FromPrimitive)]
enum InstructionCode {
    Add = 1,
    Mult = 2,
    Terminate = 99
}

#[derive(Debug)]
struct InvalidInstructionInfo {
    pub invalid_instruction: Value,
    pub instruction_location: Address,
}

#[derive(Debug)]
struct InvalidAddressInfo {
    pub invalid_address: Address,
    pub address_location: Address,
}

#[derive(Debug)]
enum Error {
    InvalidInstruction { instruction_value: Value, instruction_location: Address },
    InvalidAddress { invalid_address: Address, address_location: Address },
    AddressOutOfRange(Address),
}

struct Machine<'a> {
    memory: &'a mut Memory,
    ip: Address,
}

impl Machine<'_> {
    fn pop_address(&mut self) -> Result<Address, Error> {
        let a = self.read_address(self.ip)?;    
        self.ip.0 += 1;
        Ok(a)
    }

    fn pop_value(&mut self) -> Result<Value, Error> {
        let v = Value(self.memory[self.ip.0]);
        self.ip.0 += 1;
        Ok(v)
    }

    fn pop_instruction_code(&mut self) -> Result<InstructionCode, Error> {
        let numeric_value = self.memory[self.ip.0];
        
        match FromPrimitive::from_u64(numeric_value.try_into().unwrap()) {
            Option::Some(x) => {
                self.ip.0 += 1;
                return Ok(x);
            },
            Option::None => return Err(Error::InvalidInstruction {
                instruction_value: Value(numeric_value),
                instruction_location: self.ip,
            })
        }
    }

    fn pop_instruction(&mut self) -> Result<Instruction, Error> {
        match self.pop_instruction_code()? {
            InstructionCode::Add => Ok(Instruction::Add{ arg1: self.pop_address()?, arg2: self.pop_address()?, out: self.pop_address()?}),
            InstructionCode::Mult => Ok(Instruction::Mult{ arg1: self.pop_address()?, arg2: self.pop_address()?, out: self.pop_address()?}),
            InstructionCode::Terminate => Ok(Instruction::Terminate),
        }
    }

    fn read_value(&self, address: Address) -> Result<Value, Error> {
        Ok(Value(self.memory[address.0]))
    }

    fn read_address(&self, address: Address) -> Result<Address, Error> {
        let a = Address(self.memory[address.0].try_into().unwrap());
        if a.0 < 0 || a.0 >= self.memory.len() {
            return Err(Error::InvalidAddress {
                invalid_address: a,
                address_location: address,
            })
        }
        Ok(a)
    }

    fn set_value(&mut self, address: Address, value: Value) -> Result<(), Error> {
        self.memory[address.0] = value.0;
        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
        match instruction {
            Instruction::Add { arg1: i1, arg2: i2, out: o } => {
                self.set_value(o, Value(self.read_value(i1)?.0 + self.read_value(i2)?.0))?;
            },
            Instruction::Mult { arg1: i1, arg2: i2, out: o } => {
                self.set_value(o, Value(self.read_value(i1)?.0 * self.read_value(i2)?.0))?;
            },
            Instruction::Terminate => {
                panic!("Terminate instruction can't be executed");
            }
        }
        Ok(())
    }
}


fn execute(program: &mut [u32]) {
    let mut m = Machine { memory: program, ip: Address(0) };

    loop {
        let i = m.pop_instruction().unwrap();
        println!("{:?}", i);
        if (i == Instruction::Terminate) {
            break;
        }
        m.execute(i);
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
