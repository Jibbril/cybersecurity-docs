use rand::Rng;

// Chosen value for group creation
const P: i64 = 23;

// Chosen generator
const G: i64 = 5;

fn main() {
    // Message to send
    let m = 15;

    // Init participants
    let alice = Participant {
        name: "Alice".to_string(),
        sk: 0,
        pk: None,
    };

    let eve = Participant {
        name: "Eve".to_string(),
        sk: 0,
        pk: None,
    };

    let mut bob = Participant {
        name: "Bob".to_string(),
        sk: 6,
        pk: None, 
    };
    bob.generate_pk();

    let bob_pk = bob.get_pk();
    let c = alice.encrypt_message(m, bob_pk);

    eve.eavesdrop(c);

    bob.decrypt_message(c);
}

struct Participant {
    pub name: String,
    pub sk: i64,
    pub pk: Option<i64>,
}

impl Participant {
    fn generate_pk(&mut self) {
        self.pk = Some(mod_exp(G, self.sk, P));
    }

    fn get_pk(&self) -> i64 {
        self.pk.expect("Expected public key")
    }

    fn encrypt_message(&self, m: i64, pk: i64) -> (i64,i64) {
        let k = generate_k(P);
        let c1 = mod_exp(G, k, P);
        let s = mod_exp(pk, k, P);
        let c2 = (s*m) % P;

        println!("{} encrypted message {} into ciphertext ({},{})", self.name, m, c1,c2);

        (c1,c2)
    }

    fn decrypt_message(&self, (c1,c2): (i64,i64)) {
        let s_dec = mod_exp(c1, self.sk, P);
        let s_inv = mod_inverse(s_dec, P);
        let m_dec = (c2 * s_inv) % P;

        println!("{} decrypted ciphertext ({},{}) into message {}", self.name, c1, c2, m_dec);
    }

    fn eavesdrop(&self, (c1,c2): (i64,i64)) {
        println!("{} eavesdropped ciphertext ({},{})", self.name, c1,c2);
    }
}

fn mod_exp(base: i64, exponent: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus; 
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }

    result
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, m, a);
    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        r = r - quotient * new_r;
        (t, new_t) = (new_t, t);
        (r, new_r) = (new_r, r);
    }
    if t < 0 {
        t += m;
    }
    t
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn generate_k(p: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let p_minus_1 = p - 1;

    loop {
        let k = rng.gen_range(1..p_minus_1); // Random k in range [1, p-2]
        if gcd(k, p_minus_1) == 1 {
            return k;
        }
    }
}
