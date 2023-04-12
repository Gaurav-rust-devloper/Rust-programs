fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 3..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num1 = 7;
    let num2 = 12;
    if is_prime(num1) {
        println!("{} is prime", num1);
    } else {
        println!("{} is not prime", num1);
    }
    if is_prime(num2) {
        println!("{} is prime", num2);
    } else {
        println!("{} is not prime", num2);
    }
}
 