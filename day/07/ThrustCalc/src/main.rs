use intcode::*;

fn feedback_calc(input: &Memory, combo: &[i32;5]) -> Value {

    let mut machines: Vec<Machine> = Vec::new();

    machines.push(Machine::new(input));
    machines.push(Machine::new(input));
    machines.push(Machine::new(input));
    machines.push(Machine::new(input));
    machines.push(Machine::new(input));
    
    // Give each machine their setting input

    for i in 0..5 {
        machines[i].input().send(Value(combo[i])).unwrap();
    }

    //Kick off the initial input
    machines[0].input().send(Value(0)).unwrap();
    
    let mut outvalue = Value(0);

    // execute feedback loop
    loop {

        let mut executed = false;
        for i in 0..5 {
            match machines[i].run() {
                Err(e) if e == Error::InputNotAvailable => {
                    executed = true;
                },
                Ok(()) => {
                    continue;
                },
                Err(e) if e == Error::Terminated => {
                },
                Err(e) => {
                    panic!("Machine {} error {:?}", i, e);
                }
            }

            let next = (i + 1) % 5;
            let next_input = machines[next].input().clone();
            for v in machines[i].output().as_ref().unwrap().try_iter() {
                if i == 4 {
                    outvalue = v;
                }
                next_input.send(v).unwrap();
            }
        }

        if !executed {
            break;
        }
    }

    outvalue
}

fn max_thrust_feedback_calc(input: &Memory) -> ([i32; 5], Value) {
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

                        let outvalue = feedback_calc(input, &combo);
                        
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
