use num::integer::lcm;

fn main() {

    println!("Problem 2:\n\t{:?}", p_2());
    println!("Problem 5:\n\t{:?}", p_5());
    println!("Problem 6:\n\t{:?}", p_6());
}

/// Each new term in the Fibonacci sequence is generated by adding the previous 
/// two terms. By starting with 1 and 2, the first 10 terms will be:
/// 
/// $$
/// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ... 
/// $$
///
/// By considering the terms in the Fibonacci sequence whose values do not 
/// exceed four million, find the sum of the even-valued terms.
fn p_2() -> i64 {
    Fibonacci::new()
        .skip(1)
        .take_while(|n| n < &4_000_000)
        .filter(|n| n % 2 == 0)
        .sum()

}

#[derive(Debug)]
struct Fibonacci {
    a: i64,
    b: i64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a: 1,
            b: 0,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let swap = self.b;
        self.b = self.a;
        self.a = self. a + swap;
        Some(self.b)
    }
}




/// 2520 is the smallest number that can be divided by each of the numbers from 
/// 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the 
/// numbers from 1 to 20?
///
///
/// *Note, this is just really another of way of asking what the lcm(0..=20) is.*
fn p_5() -> i64 {
    (1..=20).fold(1, |res, n| lcm(res, n))
}

/*
/// Test if all integers 1..=n divide x
fn evenly_divisable_for_range(x: i64, n: i64) -> bool {
    (1..=n)
        .map(|a| x % a == 0)
        .all(|b| b)
}
*/

/// <p>The sum of the squares of the first ten natural numbers is,</p>
///
/// $$
/// 1^2 + 2^2 + ... + 10^2 = 385
/// $$
///
/// <p>The square of the sum of the first ten natural numbers is,</p>
///
/// $$
/// (1 + 2 + ... + 10)^2 = 55^2 = 3025
/// $$
///
/// <p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
/// <p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
fn p_6() -> i64 {
    let sum_of_squares = (100 * (100 + 1) * (2* (100) + 1)) / 6;
    let sum = (100 * (100 + 1)) / 2;
    let square_of_sum = i64::pow(sum, 2);
    square_of_sum - sum_of_squares
}




/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10,001st prime number?
fn p_7() -> i64 {

}

struct Primes {
    curr: i64,
    next: i64,
}

impl Primes {
    fn new() -> Self {
        Primes {
            curr: 2,
            next: 3,
        }
    } 
}

impl Iterator for Primes {

}
