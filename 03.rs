// problem 03: Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn square(x: int) -> int { x * x }

fn sqrt(x: int) -> int { (x as f64).sqrt() as int }

fn is_factor(x: int, y: int) -> bool { y % x == 0 }

fn find_divisor(x: int, attempt: int) -> int {
    if square(attempt) > x { x }
    else if is_factor(attempt, x) { attempt }
    else {
        find_divisor(x, attempt + 1)
    }
}

fn smallest_divisor(x: int) -> int { find_divisor(x, 2) }

fn is_prime(x: int) -> bool { x == smallest_divisor(x) }

fn main() {
    let num: int = 600851475143;
    for n in range(1, sqrt(num)).rev() {
        if is_factor(n, num) && is_prime(n) {
            println!("{} is the largest prime factor of {}", n, num);
            break;
        }
    }
}
