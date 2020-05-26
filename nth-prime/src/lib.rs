pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;
    // p will be the returned value
    let mut p: u32 = 0;
    // lookup for primes < 10; 2 and 5 are notable exceptions for heuristics
    let mut primes: Vec<u32> = Vec::with_capacity(n);
    primes.extend([2, 3, 5, 7].iter().copied());

    // use lookup if n < 5
    if n < 5 {
        let nidx = n-1; // index must be usize
        p = primes[nidx];
    }
    else {
        // the value that will be incremented to test for prime-ness
        let mut x: u32 = 7;

        while primes.len() < n {
            x += 2;
            
            // if x %
        }
    }

    // return solution p
    p
}