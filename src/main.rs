use std::io;


fn primes(first: i32, last: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let current: i32 = first;

    while current <= last
    {
        if is_prime(current) {
            res.push(current);
        }
    }
    res
}


fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0{
            return false;
        }
    }
    true
}


fn pair_count(primes: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for i in 1..primes.len() {
        if primes[i] - primes[i-1] == 2 {
            count += 1;
        }
    }
    count
}


fn main() {
    let mut input = String::new();

    let mut m: i32;
    let mut n: i32;

    io::stdin().read_line(&mut input).expect("TODO: panic message");

    println!("{}", input.trim());
}
