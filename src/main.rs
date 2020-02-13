extern crate integer_sqrt;
// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

fn main() {
    let mut prime_numbers = Vec::new();
    prime_numbers.push(2);
    for num in 3..200001 {
        if prime_numbers.len() == 10002{
            break;
        }
        let mut prime = true;
        for num_check in 2..num.integer_sqrt() {
            if num % num_check == 0 {
                prime = false;
                break;
            }
        }
        match prime {
            true => prime_numbers.push(num),
            false => continue,
        }
    }
    println!("{}", prime_numbers[10001]);
}
