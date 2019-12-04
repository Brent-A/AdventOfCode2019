mod test {
    use super::*;


}

fn get_digits(n: u32) -> [u32; 6] {
    let a = n % 10;
    let n = n / 10;
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

fn check_password(password: u32) -> bool {
    let digits = get_digits(password);

    /*
    // Part 1
    let double = 
        digits[0] == digits[1] ||
        digits[1] == digits[2] ||
        digits[2] == digits[3] ||
        digits[3] == digits[4] ||
        digits[4] == digits[5];
*/

    // Part 2
    let double = 
        (digits[0] == digits[1] && digits[1] != digits[2]) ||
        (digits[1] == digits[2] && digits[2] != digits[3] && digits[0] != digits[1]) ||
        (digits[2] == digits[3] && digits[3] != digits[4] && digits[1] != digits[2]) ||
        (digits[3] == digits[4] && digits[4] != digits[5] && digits[2] != digits[3]) ||
        digits[4] == digits[5] && digits[3] != digits[4];

    if !double {
        return false;
    }

    let decreasing = 
        digits[0] > digits[1] ||
        digits[1] > digits[2] ||
        digits[2] > digits[3] ||
        digits[3] > digits[4] ||
        digits[4] > digits[5];
    
    if decreasing {
        return false;
    }

    return true;
}

fn main() {

    let mut valid = 0;
    for p in 284639..748759 {
        if check_password(p) {
            valid += 1;
        }
    }
    println!("Valid passwords {}!", valid);
}
