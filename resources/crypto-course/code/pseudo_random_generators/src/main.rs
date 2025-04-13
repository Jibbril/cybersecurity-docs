/// Small implementation of the RSA PRG based on the least significant bit that 
/// is shown in the lecture slides
fn main() {
    // In reality these primes would be much larger
    let p = 10007;
    let q = 10009;
    let modulus = p * q;
    let exponent = 65537; // Must be coprime to modulus

    let x = 5; // Initial value/key of length shorter than l
    let l: u64 = 128; // Length of desired key

    let mut base = x;
    let mut bits = vec![least_significant_bit(base)];

    for _ in 0..l-1 {
        base = mod_exp(base, exponent, modulus);
        let lsb = least_significant_bit(base);
        bits.push(lsb);
    }
    
    let bit_string: String = bits.iter()
        .map(|&b| if b == 1 { '1' } else { '0' })
        .collect();

    println!("RSA PRG result: {}", bit_string);
}

/// Utility function to extract the least significant bit
fn least_significant_bit(num: u64) -> u64 {
    num & 1
}

/// Utility function to perform modular exponentiation
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut res = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus;
        }
        
        exp = exp / 2;

        base = (base * base) % modulus;
    }

    res
}
