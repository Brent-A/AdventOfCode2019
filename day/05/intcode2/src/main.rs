#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d2_example1() {
        let mut program = [1,0,0,0,99];
        execute(&mut program);
        assert_eq!([2,0,0,0,99], program);
    }

    #[test]
    fn d2_example2() {
        let mut program = [2,3,0,3,99];
        execute(&mut program);
        assert_eq!([2,3,0,6,99], program);
    }

    #[test]
    fn d2_example3() {
        let mut program = [2,4,4,5,99,0];
        execute(&mut program);
        assert_eq!([2,4,4,5,99,9801], program);
    }

    #[test]
    fn d2_example4() {
        let mut program = [1,1,1,4,99,5,6,0,99];
        execute(&mut program);
        assert_eq!([30,1,1,4,2,5,6,0,99], program);
    }
}

fn get_digits(n: i32) -> [u32; 6] {

    let n :u32 = n.try_into().unwrap();
    let a = n % 100;
    let n = n / 100;
    let b = n % 10;
    let n = n / 10;
    let c = n % 10;
    let n = n / 10;
    let d = n % 10;
    let n = n / 10;
    let e = n % 10;
    let n = n / 10;
    let f = n % 10;
    let n = n / 10;
    [f, e, d, c, b, a]
}

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::convert::TryInto;

#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;

type Integer = i32;
type Memory = [Integer];

#[derive(Copy, Clone, Debug, PartialEq)]
struct Address(usize);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Value(Integer);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Arg {
    Address(Address),
    Value(Value),
}


#[derive(FromPrimitive)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add { arg1: Arg, arg2: Arg, out: Arg},
    Mult { arg1: Arg, arg2: Arg, out: Arg},
    Input { out: Arg },
    Output { arg1: Arg },
    JumpIfTrue { cond: Arg, dest: Arg },
    JumpIfFalse { cond: Arg, dest: Arg },
    LessThan{ c1: Arg, c2: Arg, out: Arg },
    Equals { c1: Arg, c2: Arg, out: Arg},
    Terminate,
}

#[derive(FromPrimitive)]
enum InstructionCode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Terminate = 99
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

    
    fn pop_argument(&mut self,  mode : ParameterMode) -> Result<Arg, Error> {
        match mode {
            ParameterMode::Position => {
                Ok(Arg::Address(self.pop_address()?))
            }
            ParameterMode::Immediate => {
                Ok(Arg::Value(self.pop_value()?))
            }
        }
    }

    fn pop_instruction_code(&mut self) -> Result<(ParameterMode, ParameterMode, ParameterMode, InstructionCode), Error> {
        let numeric_value = self.memory[self.ip.0];
        
        let digits = get_digits(numeric_value);

        let numeric_opcode = digits[5];

        match FromPrimitive::from_u64(numeric_opcode.try_into().unwrap()) {
            Option::Some(x) => {
                self.ip.0 += 1;

                return Ok((FromPrimitive::from_u32(digits[2]).unwrap(),
                FromPrimitive::from_u32(digits[3]).unwrap(),
                FromPrimitive::from_u32(digits[4]).unwrap(),
                x));
            },
            Option::None => return Err(Error::InvalidInstruction {
                instruction_value: Value(numeric_value),
                instruction_location: self.ip,
            })
        }
    }

    fn pop_instruction(&mut self) -> Result<Instruction, Error> {

        let (a3, a2, a1, i) = self.pop_instruction_code()?;
        match i {
            InstructionCode::Add => Ok(Instruction::Add{ arg1: self.pop_argument(a1)?, arg2: self.pop_argument(a2)?, out: self.pop_argument(a3)?}),
            InstructionCode::Mult => Ok(Instruction::Mult{ arg1: self.pop_argument(a1)?, arg2: self.pop_argument(a2)?, out: self.pop_argument(a3)?}),
            InstructionCode::Input => Ok(Instruction::Input{ out: self.pop_argument(a1)? }),
            InstructionCode::Output => Ok(Instruction::Output{ arg1: self.pop_argument(a1)? }),
            InstructionCode::JumpIfTrue => Ok(Instruction::JumpIfTrue{ cond: self.pop_argument(a1)?, dest: self.pop_argument(a2)? }),
            InstructionCode::JumpIfFalse => Ok(Instruction::JumpIfFalse{ cond: self.pop_argument(a1)?, dest: self.pop_argument(a2)? }),
            InstructionCode::LessThan => Ok(Instruction::LessThan {c1: self.pop_argument(a1)?, c2: self.pop_argument(a2)?, out: self.pop_argument(a3)?}),
            InstructionCode::Equals => Ok(Instruction::Equals {c1: self.pop_argument(a1)?, c2: self.pop_argument(a2)?, out: self.pop_argument(a3)?}),
            InstructionCode::Terminate => Ok(Instruction::Terminate),
        }
    }


    fn read_value(&self, a: Arg) -> Result<Value, Error> {
        match a {
            Arg::Address(address) => {
                Ok(Value(self.memory[address.0]))
            },
            Arg::Value(value) => {
                Ok(value)
            } 
        }
    }

    fn read_address(&self, address: Address) -> Result<Address, Error> {
        let m = self.memory[address.0];
        let a = Address(m.try_into().unwrap());
        if a.0 < 0 || a.0 >= self.memory.len() {
            return Err(Error::InvalidAddress {
                invalid_address: a,
                address_location: address,
            })
        }
        Ok(a)
    }

    fn set_value(&mut self, arg: Arg, value: Value) -> Result<(), Error> {
        match arg {
            Arg::Address(address) => {
                self.memory[address.0] = value.0;
            },
            _  => {
                panic!("Invalid set value");
            }
        }
        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) -> Result<Option<Value>, Error> {
        match instruction {
            Instruction::Add { arg1, arg2, out } => {
                self.set_value(out, Value(self.read_value(arg1)?.0 + self.read_value(arg2)?.0))?;
            },
            Instruction::Mult { arg1, arg2, out } => {
                self.set_value(out, Value(self.read_value(arg1)?.0 * self.read_value(arg2)?.0))?;
            },
            Instruction::Input { out } => {
                self.set_value(out, Value(5))?;
            },
            Instruction::Output { arg1 } => {
                let v = self.read_value(arg1)?;
                println!("OUTPUT: {:?}", v);
                return Ok(Option::Some(v));
            },
            Instruction::JumpIfTrue { cond, dest } => {
                if self.read_value(cond)?.0 != 0 {
                    self.ip.0 = self.read_value(dest)?.0.try_into().unwrap();
                }
            },
            Instruction::JumpIfFalse { cond, dest } => {
                if self.read_value(cond)?.0 == 0 {
                    self.ip.0 = self.read_value(dest)?.0.try_into().unwrap();
                }
            },
            Instruction::LessThan { c1, c2, out } => {
                if self.read_value(c1)?.0  < self.read_value(c2)?.0  {
                    self.set_value(out, Value(1))?;
                }
                else {
                    self.set_value(out, Value(0))?;
                }
            },
            Instruction::Equals { c1, c2, out } => {
                if self.read_value(c1)?.0  == self.read_value(c2)?.0  {
                    self.set_value(out, Value(1))?;
                }
                else {
                    self.set_value(out, Value(0))?;
                }
            },
            Instruction::Terminate => {
                panic!("Terminate instruction can't be executed");
            }
        }
        Ok(Option::None)
    }
}

fn execute(program: &mut [i32]) -> Vec<Value> {
    let mut m = Machine { memory: program, ip: Address(0) };

    let mut output : Vec<Value> = Vec::new();

    loop {
        let i = m.pop_instruction().unwrap();
        println!("trace({}): {:?}", m.ip.0, i);
        if i == Instruction::Terminate {
            break;
        }
        let o = m.execute(i).unwrap();
        if let Option::Some(v) = o {
            output.push(v);
        }
    }

    output
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    
    let original : Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    
    let mut program = original.clone();

    let output = execute(&mut program);

    println!("Output is: {:?}", output);
}
