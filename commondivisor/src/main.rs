use std::str::FromStr;
use std::env;


fn main() {
    let mut numbers = Vec::new();
    
    // Skip the first argument which is the program name
    // Parse the remaining arguments as u64 numbers
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Please provide valid numbers as arguments"));
    }

    if numbers.is_empty() {
        eprintln!("Usage: commondivisor <number1> <number2> ... <numberN>");
        // Exit with a non-zero code to indicate an error
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is: {}", numbers, d);
}

// Function to compute the greatest common divisor (GCD) of two numbers using the Euclidean algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

