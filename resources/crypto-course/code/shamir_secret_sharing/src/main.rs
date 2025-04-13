use std::collections::HashMap;
use rand::Rng;


/// A minimal showcase of Shamir secret sharing
fn main() {
    // Set parameters for example
    let secret = 123;
    let t = 3;
    let p = 257; 
    let n_shares = 5;
    let mut rng = rand::thread_rng();

    println!("secret: {:#?}",secret);
    println!("t: {:#?}",t);
    println!("p: {:#?}",p);
    println!("n_shares: {:#?}",n_shares);

    // Initialize polynomial coefficients with secret as intercept
    let mut coefficients = vec![secret]; 
    for _ in 1..t {
        coefficients.push(rng.gen_range(0..p));
    }
    println!("Coefficients generated: {:?}",coefficients);

    // Compute the shares
    let mut shares = HashMap::new();
    for i in 1..n_shares {
        let x = i as u64;
        let y: u64 = coefficients.iter()
            .enumerate()
            .fold(0, |acc, (i, &c)| {
                (acc + c * mod_exp(x, i as u64, p) % p) % p
            });

        shares.insert(x, y);
    }
    println!("Shares generated: {:?}",shares);
    
    // Select t shares randomly to reconstruct secret
    let reconstruction_shares: Vec<(u64,u64)> = shares.iter()
        .take(t)
        .map(|(&k,&v)| (k,v))
        .collect();
    println!("Attempting secret reconstruction using shares: {:?}",reconstruction_shares);

    // Compute secret from reconstruction_shares
    let mut reconstructed_secret = 0;
    for (xi,yi) in reconstruction_shares.iter() {
        let mut numerator = 1;
        let mut denominator = 1;

        for (xj,_) in reconstruction_shares.iter() {
            if *xj == *xi {
                continue;
            }

            numerator = (numerator * (p - xj)) % p; 
            denominator = (denominator * (xi + p - xj)) % p; 
        }

        // Get inverse of denominator by applying Fermat's little theorem
        let inverse_denominator  = mod_exp(denominator, p-2, p);

        reconstructed_secret = (reconstructed_secret + (yi * numerator * (inverse_denominator % p))) % p;
    }

    println!("Reconstruced secret: {:#?}", reconstructed_secret);
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}
