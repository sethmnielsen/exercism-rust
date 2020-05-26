#![allow(unused_variables)]

pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;
    // p will be the returned value
    let mut p: u32 = 0;
    // lookup for primes < 10; 2 and 5 are notable exceptions for heuristics
    let mut primes: Vec<u32> = Vec::with_capacity(n);
    primes.extend([2, 3, 5, 7].iter().copied());

    // use lookup if n < 4
    if n < primes.len() {
        p = primes[n];
    }
    else {
        // the value that will be incremented to test for prime-ness
        let mut x: u32 = primes.last().unwrap().clone();
        let sq = (x as f32).sqrt();
        while primes.len() < n {
            x += 2;
            for prime in &primes {

            }
        }
    }

    // return solution p
    p
}
