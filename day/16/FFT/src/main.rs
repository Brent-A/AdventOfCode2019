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
    }
    v
}

fn process_phases(signal: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut next = signal.clone();
    
    println!("phase: _ {:?}", next);
    for i in 0..n {
        //println!("phase: {} {}", i, next);
        next = process_phase(&next);
        
        println!("phase: {} {:?}", i, next);
    }
    next
}
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
fn example2() {
    let signal = parse_signal(&"80871224585914546619083218645595");
    let r = process_phases(&signal, 100);
    let (first, rest) = r.split_at(8);

    assert_eq!(first, [2, 4, 1, 7, 6, 1, 7, 6]);
    
}

fn repeat_signal(signal: &Vec<i32>, repeat: usize) -> Vec<i32> {
    let  mut v = Vec::new();
    for _ in 0..repeat {
        v.append(&mut signal.clone());
    }
    v
}


/*
#[test]
fn experiment() {
    println!("");
    let signal = parse_signal(&"12345678");
    let full_signal = repeat_signal(&signal, 4);
    let r = process_phases(&full_signal, 100);
    
    println!("r: {:?}", r);
}

#[test]
fn part2_example() {
    let signal = parse_signal(&"03036732577212944063491565474664");
    let full_signal = repeat_signal(&signal, 10000);
    let r = process_phases(&full_signal, 100);
    let message_offset = signal[0] * 1000000 + 
                         signal[1] *  100000 + 
                         signal[2] *   10000 +
                         signal[3] *    1000 +
                         signal[4] *     100 +
                         signal[5] *      10 +
                         signal[6] *       1;

    let (_, rest) = r.split_at(message_offset as usize);
    let (out, _) = rest.split_at(8);

    assert_eq!(out, [8, 4, 4, 6, 2, 0, 2, 6]);
    
}
*/
fn main() {
    
    let file = std::fs::read_to_string("input.txt").unwrap();
    let signal = parse_signal(&file);
    let r = process_phases(&signal,100);
    let (first, rest) = r.split_at(8);
    println!("result: {:?}", first);
}
