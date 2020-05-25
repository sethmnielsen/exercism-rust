pub fn nth(n: u32) -> u32 {
    // lookup for primes < 10; 2 and 5 are notable exceptions for heuristics
    let primes: Vec<u32> = vec![2, 3, 5, 7]; // primes for n=1, 2, 3, or 4

    // counter to keep track of how many primes have been found
    let mut k: u32 = 0;
    // the value that will be incremented to test for prime-ness
    let mut x: u32 = 17;

    while k < n
}
