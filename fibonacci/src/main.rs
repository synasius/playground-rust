use std::io;


fn main() {
    println!("Tell me the nth fibonacci number you want:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input!");

    let input: u32 = input.trim().parse()
        .expect("Please enter a valid number!");

    println!("The fibonacci number {} is: {}: ", input, nth_fibonacci(input));
}

fn nth_fibonacci(input: u32) -> u64 {
    if input == 0 {
        return 0;
    } else if input == 1 {
        return 1;
    } else {

        let mut prev = 0u64;
        let mut current = 1u64;
        for _ in 2..input+1 {
            let next = prev + current;
            prev = current;
            current = next;
        }
        return current;
    }
}
