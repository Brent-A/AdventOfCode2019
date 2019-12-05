#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d2_example1() {
        let mut program = [1, 0, 0, 0, 99];
        execute(&mut program, &[]).unwrap();
        assert_eq!([2, 0, 0, 0, 99], program);
    }

    #[test]
    fn d2_example2() {
        let mut program = [2, 3, 0, 3, 99];
        execute(&mut program, &[]).unwrap();
        assert_eq!([2, 3, 0, 6, 99], program);
    }

    #[test]
    fn d2_example3() {
        let mut program = [2, 4, 4, 5, 99, 0];
        execute(&mut program, &[]).unwrap();
        assert_eq!([2, 4, 4, 5, 99, 9801], program);
    }

    #[test]
    fn d2_example4() {
        let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut program, &[]).unwrap();
        assert_eq!([30, 1, 1, 4, 2, 5, 6, 0, 99], program);
    }

    #[test]
    fn d5_example1() -> Result<(), ExecutionError> {
        // 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode,
        // consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Value(1), execute(&mut program, &[Value(8)])?[0]);
        let mut program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Value(0), execute(&mut program, &[Value(10)])?[0]);
        Ok(())
    }

    #[test]
    fn d5_example2() -> Result<(), ExecutionError> {
        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode,
        // consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(Value(1), execute(&mut program.clone(), &[Value(7)])?[0]);
        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(8)])?[0]);
        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(10)])?[0]);
        Ok(())
    }

    #[test]
    fn d5_example3() -> Result<(), ExecutionError> {
        // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode,
        // consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(Value(1), execute(&mut program.clone(), &[Value(8)])?[0]);
        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(0)])?[0]);
        Ok(())
    }

    #[test]
    fn d5_example4() -> Result<(), ExecutionError> {
        // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode,
        // consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(Value(1), execute(&mut program.clone(), &[Value(7)])?[0]);
        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(8)])?[0]);
        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(10)])?[0]);
        Ok(())
    }

    #[test]
    fn d5_jump() -> Result<(), ExecutionError> {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
        // 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)

        let program = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(0)])?[0]);
        assert_eq!(Value(1), execute(&mut program.clone(), &[Value(1)])?[0]);

        let program = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(Value(0), execute(&mut program.clone(), &[Value(0)])?[0]);
        assert_eq!(Value(1), execute(&mut program.clone(), &[Value(1)])?[0]);
        Ok(())
    }

    #[test]
    fn d5_larger() -> Result<(), ExecutionError> {
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(Value(999), execute(&mut program.clone(), &[Value(5)])?[0]);
        assert_eq!(Value(1000), execute(&mut program.clone(), &[Value(8)])?[0]);
        assert_eq!(Value(1001), execute(&mut program.clone(), &[Value(10)])?[0]);
        Ok(())
    }
}

fn get_digits(n: i32) -> [u32; 6] {
    let n: u32 = n.try_into().unwrap();
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

use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Add { arg1: Arg, arg2: Arg, out: Arg },
    Mult { arg1: Arg, arg2: Arg, out: Arg },
    Input { out: Arg },
    Output { arg1: Arg },
    JumpIfTrue { cond: Arg, dest: Arg },
    JumpIfFalse { cond: Arg, dest: Arg },
    LessThan { c1: Arg, c2: Arg, out: Arg },
    Equals { c1: Arg, c2: Arg, out: Arg },
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
    Terminate = 99,
}

#[derive(Debug, Copy, Clone)]
enum Error {
    InvalidInstruction {
        instruction_value: Value,
        instruction_location: Address,
    },
    InvalidAddress {
        invalid_address: Address,
        address_location: Address,
    },
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

    fn pop_argument(&mut self, mode: ParameterMode) -> Result<Arg, Error> {
        match mode {
            ParameterMode::Position => Ok(Arg::Address(self.pop_address()?)),
            ParameterMode::Immediate => Ok(Arg::Value(self.pop_value()?)),
        }
    }

    fn pop_instruction_code(
        &mut self,
    ) -> Result<(ParameterMode, ParameterMode, ParameterMode, InstructionCode), Error> {
        let numeric_value = self.memory[self.ip.0];

        let digits = get_digits(numeric_value);

        let numeric_opcode = digits[5];

        let e = Error::InvalidInstruction {
            instruction_value: Value(numeric_value),
            instruction_location: self.ip,
        };

        let x = FromPrimitive::from_u64(numeric_opcode.try_into().unwrap()).ok_or(e)?;

        self.ip.0 += 1;

        return Ok((
            FromPrimitive::from_u32(digits[2]).ok_or(e)?,
            FromPrimitive::from_u32(digits[3]).ok_or(e)?,
            FromPrimitive::from_u32(digits[4]).ok_or(e)?,
            x,
        ));
    }

    fn pop_instruction(&mut self) -> Result<Instruction, Error> {
        let (a3, a2, a1, i) = self.pop_instruction_code()?;
        match i {
            InstructionCode::Add => Ok(Instruction::Add {
                arg1: self.pop_argument(a1)?,
                arg2: self.pop_argument(a2)?,
                out: self.pop_argument(a3)?,
            }),
            InstructionCode::Mult => Ok(Instruction::Mult {
                arg1: self.pop_argument(a1)?,
                arg2: self.pop_argument(a2)?,
                out: self.pop_argument(a3)?,
            }),
            InstructionCode::Input => Ok(Instruction::Input {
                out: self.pop_argument(a1)?,
            }),
            InstructionCode::Output => Ok(Instruction::Output {
                arg1: self.pop_argument(a1)?,
            }),
            InstructionCode::JumpIfTrue => Ok(Instruction::JumpIfTrue {
                cond: self.pop_argument(a1)?,
                dest: self.pop_argument(a2)?,
            }),
            InstructionCode::JumpIfFalse => Ok(Instruction::JumpIfFalse {
                cond: self.pop_argument(a1)?,
                dest: self.pop_argument(a2)?,
            }),
            InstructionCode::LessThan => Ok(Instruction::LessThan {
                c1: self.pop_argument(a1)?,
                c2: self.pop_argument(a2)?,
                out: self.pop_argument(a3)?,
            }),
            InstructionCode::Equals => Ok(Instruction::Equals {
                c1: self.pop_argument(a1)?,
                c2: self.pop_argument(a2)?,
                out: self.pop_argument(a3)?,
            }),
            InstructionCode::Terminate => Ok(Instruction::Terminate),
        }
    }

    fn read_value(&self, a: Arg) -> Result<Value, Error> {
        match a {
            Arg::Address(address) => Ok(Value(self.memory[address.0])),
            Arg::Value(value) => Ok(value),
        }
    }

    fn read_address(&self, address: Address) -> Result<Address, Error> {
        let m = self.memory[address.0];
        let a = Address(m.try_into().unwrap());
        if a.0 < 0 || a.0 >= self.memory.len() {
            return Err(Error::InvalidAddress {
                invalid_address: a,
                address_location: address,
            });
        }
        Ok(a)
    }

    fn set_value(&mut self, arg: Arg, value: Value) -> Result<(), Error> {
        match arg {
            Arg::Address(address) => {
                self.memory[address.0] = value.0;
            }
            _ => {
                panic!("Invalid set value");
            }
        }
        Ok(())
    }

    fn execute<F>(&mut self, instruction: Instruction, input: F) -> Result<Option<Value>, Error>
    where
        F: FnOnce() -> Value,
    {
        match instruction {
            Instruction::Add { arg1, arg2, out } => {
                self.set_value(
                    out,
                    Value(self.read_value(arg1)?.0 + self.read_value(arg2)?.0),
                )?;
            }
            Instruction::Mult { arg1, arg2, out } => {
                self.set_value(
                    out,
                    Value(self.read_value(arg1)?.0 * self.read_value(arg2)?.0),
                )?;
            }
            Instruction::Input { out } => {
                self.set_value(out, input())?;
            }
            Instruction::Output { arg1 } => {
                let v = self.read_value(arg1)?;
                return Ok(Option::Some(v));
            }
            Instruction::JumpIfTrue { cond, dest } => {
                if self.read_value(cond)?.0 != 0 {
                    self.ip.0 = self.read_value(dest)?.0.try_into().unwrap();
                }
            }
            Instruction::JumpIfFalse { cond, dest } => {
                if self.read_value(cond)?.0 == 0 {
                    self.ip.0 = self.read_value(dest)?.0.try_into().unwrap();
                }
            }
            Instruction::LessThan { c1, c2, out } => {
                if self.read_value(c1)?.0 < self.read_value(c2)?.0 {
                    self.set_value(out, Value(1))?;
                } else {
                    self.set_value(out, Value(0))?;
                }
            }
            Instruction::Equals { c1, c2, out } => {
                if self.read_value(c1)?.0 == self.read_value(c2)?.0 {
                    self.set_value(out, Value(1))?;
                } else {
                    self.set_value(out, Value(0))?;
                }
            }
            Instruction::Terminate => {
                panic!("Terminate instruction can't be executed");
            }
        }
        Ok(Option::None)
    }
}

//#[derive(Debug)]
enum ExecutionError {
    InstructionDecode {
        inner: Error,
        trace: Vec<(Address, Instruction)>,
    },
    InstructionExecute {
        inner: Error,
        trace: Vec<(Address, Instruction)>,
    },
}

impl std::error::Error for ExecutionError {}
/*

println!("Execution error!");
for (ip, instruction) in trace {
    println!("  trace({}): {:?}", ip.0, instruction);
}

println!("Current ip: {}", m.ip.0);
println!("Error: {:?}", e);
panic!("Execution terminated");*/
impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::InstructionDecode { trace, inner } => {
                writeln!(f, "InstructionDecode Error")?;
                for (ip, instruction) in trace.iter() {
                    writeln!(f, "  trace({}): {:?}", ip.0, instruction)?;
                }
                writeln!(f, "Inner error: {:?}", inner)
            }
            ExecutionError::InstructionExecute { trace, inner } => {
                writeln!(f, "InstructionExecute Error")?;
                for (ip, instruction) in trace.iter() {
                    writeln!(f, "  trace({}): {:?}", ip.0, instruction)?;
                }
                writeln!(f, "Inner error: {:?}", inner)
            }
        }
    }
}

impl std::fmt::Debug for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::InstructionDecode { trace, inner } => {
                writeln!(f, "InstructionDecode Error")?;
                for (ip, instruction) in trace.iter() {
                    writeln!(f, "  trace({}): {:?}", ip.0, instruction)?;
                }
                writeln!(f, "Inner error: {:?}", inner)
            }
            ExecutionError::InstructionExecute { trace, inner } => {
                writeln!(f, "InstructionExecute Error")?;
                for (ip, instruction) in trace.iter() {
                    writeln!(f, "  trace({}): {:?}", ip.0, instruction)?;
                }
                writeln!(f, "Inner error: {:?}", inner)
            }
        }
    }
}

fn execute(program: &mut [i32], input: &[Value]) -> Result<Vec<Value>, ExecutionError> {
    let mut m = Machine {
        memory: program,
        ip: Address(0),
    };

    let mut output: Vec<Value> = Vec::new();

    let mut input_index = 0;

    let mut trace = Vec::new();

    loop {
        let ip = m.ip;
        let i = m
            .pop_instruction()
            .map_err(|e| ExecutionError::InstructionDecode {
                inner: e,
                trace: trace.clone(),
            })?;

        trace.push((ip, i));
        if i == Instruction::Terminate {
            break;
        }
        if let Some(o) = m
            .execute(i, || {
                let v = input[input_index];
                input_index += 1;
                v
            })
            .map_err(|e| ExecutionError::InstructionExecute {
                inner: e,
                trace: trace.clone(),
            })?
        {
            output.push(o);
        }
    }

    Ok(output)
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let original: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut program = original.clone();
    let output = execute(&mut program, &[Value(5)]);

    println!("Output is: {:?}", output);
}
