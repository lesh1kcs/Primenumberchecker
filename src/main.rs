fn is_prime(n: i32) -> bool{
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;    
}

fn main() {
    println!("Enter a number to check if it is prime or not:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
