use rand::Rng;

const P: u64 = 23;
const G: u64 = 2; // Not a generator of Z_p^*
const H: u64 = 6;

fn main() {
    let mut alice = Participant::new("Alice");
    let mut bob = Participant::new("Bob");

    let commitment = alice.create_commitment();
    bob.receive_commitment(commitment);

    println!("");
    println!("At a later point in time...");
    println!("");

    let (m, r) = alice.reveal_values();
    bob.validate_commitment(m, r);
}

struct Participant {
    name: String,
    m: Option<u64>,
    r: Option<u64>,
}

impl Participant {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            m: None,
            r: None,
        }
    }

    fn create_commitment(&mut self) -> u64 {
        let mut rng = rand::thread_rng();

        let m = rng.gen_range(0..(P - 1));
        let mut r = rng.gen_range(0..(P - 1));

        // Ensure m != r
        loop {
            if m == r {
                r = rng.gen_range(0..(P - 1));
                continue;
            }

            break;
        }

        let commitment = (mod_exp(G, m, P) * mod_exp(H, r, P)) % P;

        self.m = Some(m);
        self.r = Some(r);

        println!("{} created the commitment {}", self.name, commitment);

        commitment
    }

    fn receive_commitment(&mut self, c: u64) {
        self.m = Some(c);
        println!("{} received the commitment {}", self.name, c);
    }

    fn reveal_values(&self) -> (u64, u64) {
        let m = self.m.expect("Expected m");
        let r = self.r.expect("Expected r");

        println!("{} revealed the values m = {} and r = {}", self.name, m, r);

        (m, r)
    }

    fn validate_commitment(&self, m: u64, r: u64) {
        let new_c = (mod_exp(G, m, P) * mod_exp(H, r, P)) % P;
        let prev_c = self.m.expect("Expected c");

        println!(
            "{} validates message {} and randomness {} against previous commitment {}",
            self.name, m, r, prev_c
        );

        if new_c == prev_c {
            println!("Validation successful!");
        } else {
            println!("Validation failed!");
        }
    }
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }

    result
}
