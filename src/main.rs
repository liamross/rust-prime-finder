use std::io;

fn main() {
    let mut nth = String::new();

    println!("\nInput the nth prime number you wish to find.");

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line.");

    let nth = match input_parser(nth) {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let prime = find_prime_iterative(nth);
    // let prime = find_prime_recursive(1, 1, nth);

    println!("Prime number: {}", prime);
}

fn find_prime_iterative(initial: u32) -> u32 {
    let mut prime = 1;
    let mut curr = 1;
    let mut prime_count = 0;

    while prime_count < initial {
        for denom in 2..curr {
            if curr % denom == 0 {
                curr = curr + 1;
                continue;
            }
        }
        prime = curr;
        curr = curr + 1;
        prime_count = prime_count + 1;
    }

    return prime;
}

// fn find_prime_recursive(prime: u64, curr: u64, left: u32) -> u64 {
//     if left <= 0 {
//         return prime;
//     }

//     for denom in 2..curr {
//         if curr % denom == 0 {
//             return find_prime_recursive(prime, curr + 1, left);
//         }
//     }

//     find_prime_recursive(curr, curr + 1, left - 1)
// }

fn input_parser(input: String) -> Result<u32, ()> {
    return match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(()),
    };
}
