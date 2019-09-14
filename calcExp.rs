/*
    Simple "a to the power of b" calculator. 
    sep/2019
    GHC

    ATM: code does not work because: 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:999:5
    Goal: make it so that the program outputs staticly the power tables for 2,3,4,5,6,7,8,9 from exp = 0 to 5
*/

use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    
    // Read base
    println!("Input the base:");
    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    // Read exponent
    println!("Input the exponent:");
    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    // Convert input to int(s)
    let a_out: i32 = a.parse().unwrap();
    let b_out: u32 = b.parse().unwrap();

    // Calculate the exponent
    let exp = a_out.pow(b_out);

    println!("{0} ^ {1} = {2}", a_out, b_out, exp);
}