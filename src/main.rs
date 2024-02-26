fn is_prime(number: u64) -> bool {
    if number == 0 || number == 1 {
        return false;
    }
    let mut x: u64 = 2;
    while x * x <= number{
        if number % x == 0 {
            return  false;
        }
        x += 1;
    }
    true
}

fn main() {
    for x in 1u64..100u64 {
        if is_prime(x) {
            println!("{} is a prime.", x);
        }
        else {
            println!("{} is not a prime.", x);
        }
    }
}