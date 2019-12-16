const BASEPATTERN : [i32; 4] = [0, 1, 0, -1];
fn repeat_base(n: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for i in &BASEPATTERN {
        for j in 0..n {
            v.push(*i);
        }
    }
    v
}

fn parse_signal(input: &str) -> Vec<i32> {
    let mut v = Vec::new();
    for c in input.chars() {
        let chstr : String = c.to_string();
        let n = chstr.parse::<i32>().unwrap();
        v.push(n);
    }
    v
}

fn process_phase(signal: &Vec<i32>) -> Vec<i32> {

    let mut v = Vec::new();

    let len = signal.len();
    //println!("len: {}", len);

    for digit in 0..len {
        let sequence = repeat_base(digit + 1);

        let mut sum = 0;
        for col in digit..signal.len() {
            let sequence_offseta = 1;
            let sequence_indexa = (sequence_offseta + col) % sequence.len();
            let coeffa = sequence[sequence_indexa];


            let sequence_len = 4 * (digit + 1);
            let sequence_index = (col + 1) % sequence_len;
            let base_sequence_index = sequence_index / (digit + 1);
            let coeff = BASEPATTERN[base_sequence_index];

            assert_eq!(sequence_indexa, sequence_index);
            //println!("sequence_index: {}, base_sequence_index: {}, coeff: {}", sequence_index, base_sequence_index, coeff );
            assert_eq!(coeffa, coeff);

            //let val = (signal[col] % 10).abs();
            let val = signal[col];
            let colval = coeff * val;
            //println!("{}*{} + ", val, coeff);
            sum += colval;
        }

        let coloutput = (sum % 10).abs();
        //let coloutput = sum;
        //println!("= {} ", coloutput);
        v.push(coloutput);
    }
    v
}

fn process_phases(signal: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut next = signal.clone();
    
    //println!("phase: _ {:?}", next);
    for i in 0..n {
        //println!("phase: {} {}", i, next);
        next = process_phase(&next);
        
        //println!("phase: {} {:?}", i, next);
    }
    next
}

use std::collections::HashMap;
struct FFTSolver {
    signal_length : usize,
    repeats: usize,
    known_digits : HashMap<(usize, usize), i32>,
    last_stat_time: Option<std::time::Instant>,
}

impl FFTSolver {
    fn new(signal: &[i32], repeats: usize) -> Self {
        let mut map : HashMap<(usize, usize), i32> = HashMap::new();
        for (i, digit) in signal.iter().enumerate() {
            map.insert((0, i), *digit);
        }
        Self {
            signal_length: signal.len() * repeats,
            known_digits: map,
            repeats: repeats,
            last_stat_time: None,
        }
    }

    fn get_value(&mut self, iteration: usize, digit: usize) -> i32 {

        let lookup;
        if iteration == 0 {
            lookup = (iteration, digit % (self.signal_length / self.repeats));
        }
        else {
            lookup = (iteration, digit);
        }

        if let Some(v) = self.known_digits.get(&lookup) {
            return *v;
        }
        else {
            // pattern: [0, 1, 0, -1];
            
            let sequence_len = 4 * (digit + 1);

            let mut sum = 0;
            for col in digit..self.signal_length {
                let sequence_index = (col + 1) % sequence_len;
                let base_sequence_index = sequence_index / (digit + 1);
                let coeff = BASEPATTERN[base_sequence_index];

                let val;
                if coeff == 0 {
                    val = 0;
                } else {
                    val = coeff * self.get_value(iteration - 1, col);
                }

                sum = sum + val;
            }
            
            let total = (sum % 10).abs();
            self.known_digits.insert((iteration, digit), total);
            //if iteration > 1 {
            //println!("Solved i: {}, d: {} = {} (total: {})", iteration, digit, total, self.known_digits.len());
            
            //}

            if self.last_stat_time.is_none() || self.last_stat_time.unwrap().elapsed() > std::time::Duration::from_secs(1) {
                println!("Solved i: {}, d: {} = {} (total: {})", iteration, digit, total, self.known_digits.len());
                self.last_stat_time = Some(std::time::Instant::now());
            }
            return total;
        }
    }
}
/*

    for digit in 0..len {
        let sequence = repeat_base(digit + 1);

        let mut sum = 0;
        for col in digit..signal.len() {
            let sequence_offset = 1;
            let sequence_index = (sequence_offset + col) % sequence.len();
            let coeff = sequence[sequence_index];
            //let val = (signal[col] % 10).abs();
            let val = signal[col];
            let colval = coeff * val;
            //println!("{}*{} + ", val, coeff);
            sum += colval;
        }

        let coloutput = (sum % 10).abs();
        //let coloutput = sum;
        //println!("= {} ", coloutput);
        v.push(coloutput);
    }*/

#[test]
fn example() {
    let str_signal = "12345678";
    let signal = parse_signal(&str_signal);
    assert_eq!(signal, [1, 2, 3, 4, 5, 6, 7, 8]);

    let phase1 = process_phase(&signal);
    
    assert_eq!(phase1, [4, 8, 2, 2, 6, 1, 5, 8]);
    
    let phase2 = process_phase(&phase1);
    
    assert_eq!(phase2, [3, 4, 0, 4, 0, 4, 3, 8]);
}

#[test]
fn example_alt() {
    let str_signal = "12345678";
    let signal = parse_signal(&str_signal);
    assert_eq!(signal, [1, 2, 3, 4, 5, 6, 7, 8]);

    let mut solver = FFTSolver::new(&signal, 1);

    let phase1 : Vec<i32> = (0..8).map(|x| solver.get_value(1, x)).collect();
    
    assert_eq!(phase1, [4, 8, 2, 2, 6, 1, 5, 8]);
    
    let phase2 : Vec<i32>  = (0..8).map(|x| solver.get_value(2, x)).collect();
    
    assert_eq!(phase2, [3, 4, 0, 4, 0, 4, 3, 8]);
}
#[test]
fn example2() {
    let signal = parse_signal(&"80871224585914546619083218645595");
    let r = process_phases(&signal, 100);
    let (first, rest) = r.split_at(8);

    assert_eq!(first, [2, 4, 1, 7, 6, 1, 7, 6]);
    
    let mut solver = FFTSolver::new(&signal, 1);
    let result :Vec<i32> = (0..8).map(|x| solver.get_value(100, x)).collect();
    assert_eq!(result, [2, 4, 1, 7, 6, 1, 7, 6]);
}

fn repeat_signal(signal: &Vec<i32>, repeat: usize) -> Vec<i32> {
    let  mut v = Vec::new();
    for _ in 0..repeat {
        v.append(&mut signal.clone());
    }
    v
}

#[test]
fn repetition() {
    let signal = parse_signal(&"03036732577212944063491565474664");
    let full_signal = repeat_signal(&signal, 10000);

    
    let mut solver = FFTSolver::new(&signal, 10000);


    let last_digit = signal.len() * 10000 - 1;
    
    println!("");
    for x in 0..10 {
        print!("{} from end: ", x);
        let values : Vec<i32> = (0..100).map(|i| solver.get_value(i, last_digit - x)).collect();

        let mut period = 1;
        while (period < 100) {

        }
        for i in 0..100 {
            print!("{}", solver.get_value(i, last_digit - x));
        }
        println!("");
    }
}

#[test]
fn part2_example() {
    let signal = parse_signal(&"03036732577212944063491565474664");
    let full_signal = repeat_signal(&signal, 10000);
    let message_offset : usize = (signal[0] * 1000000 + 
                         signal[1] *  100000 + 
                         signal[2] *   10000 +
                         signal[3] *    1000 +
                         signal[4] *     100 +
                         signal[5] *      10 +
                         signal[6] *       1) as usize;

    let mut solver = FFTSolver::new(&signal, 10000);

    let last_digit = solver.get_value(100, signal.len() * 10000 - 1);
    println!("Solved last_digit: {}", last_digit);
    println!("Solved total digits: {}", solver.known_digits.len());

    
    let last_digit = solver.get_value(100, signal.len() * 10000 - 2);
    println!("Solved 2nd last_digit: {}", last_digit);
    println!("Solved total digits: {}", solver.known_digits.len());

    
    
    let last_digit = solver.get_value(100, signal.len() * 10000 - 3);
    println!("Solved 3rd last_digit: {}", last_digit);
    println!("Solved total digits: {}", solver.known_digits.len());

    
    let last_digit = solver.get_value(100, signal.len() * 10000 - 5);
    println!("Solved 4th last_digit: {}", last_digit);
    println!("Solved total digits: {}", solver.known_digits.len());


    let d1 = solver.get_value(100,  message_offset);
    println!("Solved d1: {}", d1);
    println!("Solved total digits: {}", solver.known_digits.len());


    let d2 = solver.get_value(100,  message_offset + 1);
    println!("Solved d2: {}", d2);
    println!("Solved total digits: {}", solver.known_digits.len());
    

    let result :Vec<i32> = (0..8).map(|x| solver.get_value(100, x + message_offset)).collect();

    //let (_, rest) = r.split_at(message_offset as usize);
    //let (out, _) = rest.split_at(8);

    assert_eq!(result, [8, 4, 4, 6, 2, 0, 2, 6]);
    
}

fn main() {
    
    let file = std::fs::read_to_string("input.txt").unwrap();
    let signal = parse_signal(&file);
    //let r = process_phases(&signal,100);
    //let (first, rest) = r.split_at(8);
    //println!("result: {:?}", first);

    let message_offset : usize = (signal[0] * 1000000 + 
        signal[1] *  100000 + 
        signal[2] *   10000 +
        signal[3] *    1000 +
        signal[4] *     100 +
        signal[5] *      10 +
        signal[6] *       1) as usize;

    let mut solver = FFTSolver::new(&signal, 10000);

    println!("original_len: {}", signal.len());
    println!("message_offset: {}", message_offset);
    let total_len = signal.len() * 10000;
    let from_len = total_len - message_offset;
    println!("total_message_len: {}",total_len);
    println!("distance to end: {}", from_len);
}
