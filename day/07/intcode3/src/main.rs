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
    fn d5_example1() -> Result<(), Error> {
        // 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode,
        // consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Value(1), execute(&mut program, &[Value(8)])?.unwrap());
        let mut program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Value(0), execute(&mut program, &[Value(10)])?.unwrap());
        Ok(())
    }

    #[test]
    fn d5_example2() -> Result<(), Error> {
        // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode,
        // consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(
            Value(1),
            execute(&mut program.clone(), &[Value(7)])?.unwrap()
        );
        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(8)])?.unwrap()
        );
        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(10)])?.unwrap()
        );
        Ok(())
    }

    #[test]
    fn d5_example3() -> Result<(), Error> {
        // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode,
        // consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(
            Value(1),
            execute(&mut program.clone(), &[Value(8)])?.unwrap()
        );
        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(0)])?.unwrap()
        );
        Ok(())
    }

    #[test]
    fn d5_example4() -> Result<(), Error> {
        // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode,
        // consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(
            Value(1),
            execute(&mut program.clone(), &[Value(7)])?.unwrap()
        );
        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(8)])?.unwrap()
        );
        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(10)])?.unwrap()
        );
        Ok(())
    }

    #[test]
    fn d5_jump() -> Result<(), Error> {
        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
        // 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)

        let program = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(0)])?.unwrap()
        );
        assert_eq!(
            Value(1),
            execute(&mut program.clone(), &[Value(1)])?.unwrap()
        );

        let program = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(
            Value(0),
            execute(&mut program.clone(), &[Value(0)])?.unwrap()
        );
        assert_eq!(
            Value(1),
            execute(&mut program.clone(), &[Value(1)])?.unwrap()
        );
        Ok(())
    }

    #[test]
    fn d5_larger() -> Result<(), Error> {
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(
            Value(999),
            execute(&mut program.clone(), &[Value(5)])?.unwrap()
        );
        assert_eq!(
            Value(1000),
            execute(&mut program.clone(), &[Value(8)])?.unwrap()
        );
        assert_eq!(
            Value(1001),
            execute(&mut program.clone(), &[Value(10)])?.unwrap()
        );
        Ok(())
    }

    /*
    #[test]
    fn amp1() {
        //15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0

        let program = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let (combo, result) = max_thrust_calc(&program);

        println!("combo: {:?}", combo);
        //assert_eq!(combo, (4,3,2,1,0));
        assert_eq!(result, Value(43210))
    }
    */

    #[test]
    fn amp2() {
        let program = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
        27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let (combo, result) = max_thrust_feedback_calc(&program);
        
        println!("combo: {:?}", combo);
        //assert_eq!(combo, (4,3,2,1,0));
        assert_eq!(result, Value(139629729))
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

#[derive(Debug)]
enum MachineState {
    DecodeInstruction,
    ExecuteInstruction(Instruction),
    WaitingForInput(Instruction),
    Terminated,
}

struct Machine {
    memory: Vec<i32>,
    ip: Address,
    state: MachineState,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum RunResult {
    NeedInput,
    Output(Value),
    Done,
}

impl Machine {
    fn new(program: &[i32]) -> Self {
        Self {
            memory: program.to_vec(),
            ip: Address(0),
            state: MachineState::DecodeInstruction,
        }
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

    fn execute(
        &mut self,
        instruction: Instruction,
        input: Option<Value>,
    ) -> Result<Option<Value>, Error> {
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
                self.set_value(out, input.unwrap())?;
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

    fn run(&mut self, input: Option<Value>) -> Result<RunResult, Error> {
        loop {
            //println!("{:p} {:?}", self, self.state);
            match self.state {
                MachineState::DecodeInstruction => {
                    self.state = MachineState::ExecuteInstruction(self.pop_instruction()?);
                    continue;
                }
                MachineState::ExecuteInstruction(i) => match i {
                    Instruction::Terminate => {
                        self.state = MachineState::Terminated;
                        return Ok(RunResult::Done);
                    }
                    Instruction::Input { out: _ } => {
                        self.state = MachineState::WaitingForInput(i);
                        return Ok(RunResult::NeedInput);
                    }
                    _ => {
                        self.state = MachineState::DecodeInstruction;
                        if let Some(output) = self.execute(i, Option::None)? {
                            return Ok(RunResult::Output(output));
                        } else {
                            continue;
                        }
                    }
                },
                MachineState::WaitingForInput(i) => {
                    self.state = MachineState::DecodeInstruction;
                    self.execute(i, input)?;
                    continue;
                }
                MachineState::Terminated => {
                    panic!("Executing terminated machine");
                }
            }
        }
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

fn execute(program: &mut [i32], input: &[Value]) -> Result<Option<Value>, Error> {
    let mut m = Machine::new(program);

    let mut input_index = 0;


    let mut s = m.run(Option::None)?;
    loop {
        match s {
            RunResult::NeedInput => {
                s = m.run(Some(input[input_index]))?;
            },
            RunResult::Output(v) => {
                
    program.copy_from_slice(&m.memory);
                return Ok(Some(v));
            },
            RunResult::Done => {
                
    program.copy_from_slice(&m.memory);
                return Ok(None);
            }
        }
    }

    program.copy_from_slice(&m.memory);
     
    Ok(None)
}

fn max_thrust_calc(input: &[i32]) -> ([i32; 5], Value) {
    let mut max: Option<([i32; 5], Value)> = Option::None;

    let mut program: Vec<i32> = input.to_vec();

    program.clone_from_slice(input);
    let program = program;

    for s0 in 0..5 {
        for s1 in 0..5 {
            for s2 in 0..5 {
                for s3 in 0..5 {
                    for s4 in 0..5 {
                        if s0 == s1
                            || s0 == s2
                            || s0 == s3
                            || s0 == s4
                            || s1 == s2
                            || s1 == s3
                            || s1 == s4
                            || s2 == s3
                            || s2 == s3
                            || s2 == s4
                            || s3 == s4
                        {
                            continue;
                        }

                        let o0 = execute(&mut program.clone(), &[Value(s0), Value(0)]).unwrap();
                        let o1 = execute(&mut program.clone(), &[Value(s1), o0.unwrap()]).unwrap();
                        let o2 = execute(&mut program.clone(), &[Value(s2), o1.unwrap()]).unwrap();
                        let o3 = execute(&mut program.clone(), &[Value(s3), o2.unwrap()]).unwrap();
                        let o4 = execute(&mut program.clone(), &[Value(s4), o3.unwrap()]).unwrap();

                        let combo = [s0, s1, s2, s3, s4];

                        if max == Option::None {
                            max = Some((combo, o4.unwrap()));
                        } else if let Some((c, m)) = max {
                            if o4.unwrap().0 > m.0 {
                                max = Some((combo, o4.unwrap()));
                            }
                        }
                    }
                }
            }
        }
    }

    max.unwrap()
}

fn max_thrust_feedback_calc(input: &[i32]) -> ([i32; 5], Value) {
    let mut max: Option<([i32; 5], Value)> = Option::None;

    let mut program: Vec<i32> = input.to_vec();

    program.clone_from_slice(input);
    let program = program;

    println!("thrustcalc");

    for s0 in 5..10 {
        for s1 in 5..10 {
            for s2 in 5..10 {
                for s3 in 5..10 {
                    for s4 in 5..10 {
                        

                        if     s0 == s1
                            || s0 == s2
                            || s0 == s3
                            || s0 == s4
                            || s1 == s2
                            || s1 == s3
                            || s1 == s4
                            || s2 == s3
                            || s2 == s4
                            || s3 == s4
                        {
                            continue;
                        }

                        //if s0 != 9 || s1 != 8 || s2 != 7 || s3 != 6 || s4 != 5 {
                        //    continue;
                        //}
                        let combo = [s0, s1, s2, s3, s4];
                        println!("trying: {:?}", combo);


                        let mut programs: Vec<Vec<i32>> = Vec::new();
                        for _ in 0..5 {
                            programs.push(program.clone());
                        }
                        let mut machines: Vec<Machine> = Vec::new();
                        for i in 0..5 {
                            machines.push(Machine::new(&mut programs[i]));
                        }

                        let mut runstates = [RunResult::Done, 
                        RunResult::Done, 
                        RunResult::Done, 
                        RunResult::Done, 
                        RunResult::Done ];

                        

                        let mut outputs : [Option<Value>; 5] = [None, None, None, None, Some(Value(0))];
                        
                        // Give each machine their setting input
                        
                        
                        //println!("I {:?} {:?}",  runstates, outputs);

                        for i in 0..5 {
                            runstates[i] = machines[i].run(Option::None).unwrap();
                            assert_eq!(runstates[i], RunResult::NeedInput);
                            //println!("I2 {:?} {:?}",  runstates, outputs);
                            runstates[i] = machines[i].run(Some(Value(combo[i]))).unwrap();
                            //assert_eq!(runstates[i], RunResult::NeedInput);
                        }

           
                        //println!("R {:?} {:?}",  runstates, outputs);


                        let mut outvalue = Value(0);
                        // execute feedback loop
                        loop {

                            let mut executed = false;
                            for i in 0..5 {
                                //let next = (i + 1) % 5;
                                let prev;
                                if i == 0 {
                                    prev = 4;
                                }
                                else {
                                    prev = i - 1;
                                }

                                match runstates[i] {
                                    RunResult::Output(value) => {
                                        //println!("M({}) output {:?} {:?} {:?}", i, value, runstates, outputs);
                                        outputs[i] = Some(value);
                                        if i == 4 {
                                            outvalue = value; 
                                        }
                                        runstates[i] = machines[i].run(Option::None).unwrap();
                                        executed = true;
                                    },
                                    RunResult::NeedInput => {
                                        if let Some(value) = outputs[prev] {
                                            //println!("M({}) input {:?} {:?} {:?}", i, value, runstates, outputs);
                                            runstates[i] = machines[i].run(Some(value)).unwrap();
                                            outputs[prev] = None;
                                            executed = true;
                                        }
                                    },
                                    RunResult::Done => {
                                        //println!("M({}) done", i);
                                    }
                                }
                            }

                            if !executed {
                                break;
                            }
                        }

                        
                        //println!("F {:?} {:?}",  runstates, outputs);


                        if max == Option::None {
                            max = Some((combo, outvalue));
                        } else if let Some((c, m)) = max {
                            if outvalue.0 > m.0 {
                                max = Some((combo, outvalue));
                            }
                        }
                    }
                }
            }
        }
    }

    max.unwrap()
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let original: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut program = original.clone();

    let output = max_thrust_feedback_calc(&mut program);
    println!("Output is: {:?}", output);
}
