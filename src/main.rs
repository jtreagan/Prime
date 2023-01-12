// Find and print all prime numbers within a given range.

use prime::numput::inpt_u64;
use prime::porc;

fn main() {
    println!("\nWhat range of numbers do you wish to check for primes?");
    println!("Please enter your starting number:    ");
    let mut start = inpt_u64();
    let beginning = start;
    println!("Please enter your ending number:    ");
    let end = inpt_u64();

    let mut ham: (bool, u64);
    let mut sausage = 0;

    while start <= end {
        ham = porc(start);
        if ham.0 == true {
            println!("{}    is a prime number.", start);
            sausage += 1;
        }
        start += 1;

        if start % 10 == 0 {
            // Nothing to do with finding primes.  This is feedback.
            println!("Working on {} and higher.", start);
        }
    }
    println!(
        "\nThere are  {}  primes between  {} and {}.",
        sausage, beginning, end
    );
}
