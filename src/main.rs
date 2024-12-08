use std::io;


fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0{
            return false;
        }
    }
    true
}


fn pair_count(first: i32, last: i32) -> i32 {
    if first >= last || first <= 0 || last <= 0 {
        return 0;
    }

    let mut count: i32 = 0;
    let mut prev: i32 = 0;
    for i in first..last+1 {
        if is_prime(i){
            if prev != 0 && i - prev == 2 {
                count += 1;
            }
            prev = i;
        }
    }
    count
}


fn main() {
    let mut input = String::new();

    let m: i32;
    let n: i32;

    io::stdin().read_line(&mut input).unwrap();
    m = input.trim().parse::<i32>().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    n = input.trim().parse::<i32>().unwrap();

    println!("{}", pair_count(m, n));
}
