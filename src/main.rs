use std::io;

fn main() {
    println!("\nInput the nth prime number you wish to find.");

    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line.");

    let nth: u32 = match nth.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let prime = find_prime_iterative(nth);

    println!("Prime number: {}", prime);
}

fn find_prime_iterative(initial: u32) -> u32 {
    let mut prime = 1;
    let mut curr = 1;
    let mut prime_count = 0;

    while prime_count < initial {
        let mut denom = 2;
        while denom < curr {
            if curr % denom == 0 {
                curr = curr + 1;
                continue;
            }
            denom += 1;
        }

        prime = curr;
        curr = curr + 1;
        prime_count = prime_count + 1;
    }

    return prime;
}
