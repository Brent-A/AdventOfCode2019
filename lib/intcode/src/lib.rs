#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::convert::TryInto;

#[macro_use]
extern crate num_derive;
use num_traits::FromPrimitive;

pub type Integer = i64;
pub type Memory = [Integer];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Address(pub usize);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value(pub Integer);


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Arg {
    Address(Address),
    Value(Value),
    Offset(Value),
}

#[derive(FromPrimitive)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Add { arg1: Arg, arg2: Arg, out: Arg },
    Mult { arg1: Arg, arg2: Arg, out: Arg },
    Input { out: Arg },
    Output { arg1: Arg },
    JumpIfTrue { cond: Arg, dest: Arg },
    JumpIfFalse { cond: Arg, dest: Arg },
    LessThan { c1: Arg, c2: Arg, out: Arg },
    Equals { c1: Arg, c2: Arg, out: Arg },
    RelBaseAdjust { amount: Arg },
    Terminate,
}

#[derive(FromPrimitive)]
pub enum InstructionCode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    RelBaseAdjsust = 9,
    Terminate = 99,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    InvalidInstruction {
        instruction_value: Value,
        instruction_location: Address,
    },
    InvalidAddress {
        invalid_address: Address,
        address_location: Address,
    },
    InputNotAvailable,
    Terminated,
}

#[derive(Debug)]
enum MachineState {
    DecodeInstruction,
    ExecuteInstruction(Instruction),
    Terminated,
}

    fn usize_add(a : usize,  b: Integer) -> usize {
        let signed : isize = a.try_into().unwrap();
        let offset : isize = b.try_into().unwrap();
        (signed + offset).try_into().unwrap()
    }



fn get_digits(n: Integer) -> [Integer; 6] {
    let n: Integer = n.try_into().unwrap();
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
    [f, e, d, c, b, a]
}

use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Debug)]
pub struct Machine {
    memory: Vec<Integer>,
    ip: Address,
    relbase: Address,
    state: MachineState,
    input_tx: Sender<Value>,
    input: Receiver<Value>,
    output: Sender<Value>,
    output_rx: Option<Receiver<Value>>,
    block_for_input: bool,
}

impl Machine {

    pub fn new(program: &Memory) -> Self {
        let (tx0, rx0) = channel();
        let (tx1, rx1) = channel();
        Self {
            memory: program.to_vec(),
            ip: Address(0),
            relbase: Address(0),
            state: MachineState::DecodeInstruction,
            input: rx0,
            output: tx1,
            block_for_input: false,
            input_tx: tx0,
            output_rx: Some(rx1),
        }
    }

    pub fn input(&self) -> &Sender<Value> {
        &self.input_tx
    }

    pub fn output(&mut self) -> &mut Option<Receiver<Value>> {
        &mut self.output_rx
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

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
            ParameterMode::Relative => Ok(Arg::Offset(self.pop_value()?))
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
            FromPrimitive::from_i64(digits[2]).ok_or(e)?,
            FromPrimitive::from_i64(digits[3]).ok_or(e)?,
            FromPrimitive::from_i64(digits[4]).ok_or(e)?,
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
            InstructionCode::RelBaseAdjsust => Ok(Instruction::RelBaseAdjust {
                amount: self.pop_argument(a1)?,
            }),
            InstructionCode::Terminate => Ok(Instruction::Terminate),
        }
    }

    fn read_value(&self, a: Arg) -> Result<Value, Error> {
        let read_address;
        match a {
            Arg::Value(value) => {
                return Ok(value);
            },
            Arg::Address(address) => { read_address = address.0; },
            
            Arg::Offset(offset) => { read_address = usize_add(self.relbase.0, offset.0); }//Ok(Value(self.memory[address.0 + self.relbase.0]),)
        }

        if read_address >= self.memory.len() {
            return Ok(Value(0));
        }
        else {
            return Ok(Value(self.memory[read_address]));
        }
    }

    fn read_address(&self, address: Address) -> Result<Address, Error> {
        let m = self.memory[address.0];
        let a = Address(m.try_into().unwrap());
        Ok(a)
    }

    fn set_value(&mut self, arg: Arg, value: Value) -> Result<(), Error> {
        let write_address;
        match arg {
            Arg::Address(address) => {
                write_address = address.0;
            },
            Arg::Offset(offset) => {
                write_address = usize_add(self.relbase.0, offset.0);
            },
            Arg::Value(_) => {
                panic!("Invalid set value");
            }
        }

        if self.memory.len() <= write_address {
            self.memory.resize(write_address + 1, 0);
        }

        self.memory[write_address] = value.0;

        Ok(())
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<(), Error> {
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

                let input;
                if self.block_for_input {
                    input = self.input.recv().unwrap();
                } else {
                    let result = self.input.try_recv();

                    if let Err(recv_err) = result {
                        if recv_err == std::sync::mpsc::TryRecvError::Empty {
                            return Err(Error::InputNotAvailable);
                        }
                    }

                    input = result.unwrap();
                }
                
                self.set_value(out, input)?;
            }
            Instruction::Output { arg1 } => {
                let v = self.read_value(arg1)?;
                self.output.send(v).unwrap();
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
            },
            Instruction::RelBaseAdjust { amount } => {
                let offset = self.read_value(amount)?.0;
                if offset < 0 {
                    self.relbase.0 -= (-offset) as usize;
                }
                else {
                    self.relbase.0 += offset as usize;
                }
            },
            Instruction::Terminate => {
                self.state = MachineState::Terminated;
                return Err(Error::Terminated);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            //println!("{:p} {:?},{:?} {:?}", self, self.ip, self.relbase, self.state);
            //println!("  {:?}", self.memory);
            match self.state {
                MachineState::DecodeInstruction => {
                    self.state = MachineState::ExecuteInstruction(self.pop_instruction()?);
                }
                MachineState::ExecuteInstruction(i) => {
                    self.execute_instruction(i)?;
                    self.state = MachineState::DecodeInstruction;
                },
                MachineState::Terminated => {
                    return Err(Error::Terminated);
                }
            }
        }
    }
}
