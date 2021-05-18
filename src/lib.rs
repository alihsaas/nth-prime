pub fn get_primes(n: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..n + 1).map(|num| num == 2 || num & 1 != 0).collect();
    primes[1] = false;

    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 3..sqrt_n {
        if primes[i] {
            let mut j = i * i;
            while j <= n {
                primes[j] = false;
                j += i;
            }
        }
    }
    primes.iter().enumerate().filter(|a| a.1 == &true).map(|a| a.0).collect::<Vec<_>>()
}


pub fn get_nth_prime(n: usize) -> usize {
    let mut size_factor = 3;
    let mut s = n * size_factor;
    let mut primes = get_primes(s);

    while primes.len() < n {
        primes = get_primes(s);
        size_factor += 1;
        s = n * size_factor;
    }

    primes[n]
}