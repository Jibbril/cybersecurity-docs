use rand::Rng;

const G: u64 = 2; // Generator
const P: u64 = 23; // Prime number, much larger in real implementation

/// Minimal example of the Schnorr Sigma-Protocol
fn main() {
    let mut prover = Participant::new("Prover");
    let mut verifier = Participant::new("Verifier");

    let h = prover.generate_keys();
    verifier.set_h(h);

    let a = prover.get_commitment();
    verifier.receive_commitment(a);

    let e = verifier.generate_challenge();
    prover.receive_challenge(e);

    let z = prover.calculate_response();
    verifier.receive_response(z);
}

struct Participant {
    name: String,
    a: Option<u64>,
    e: Option<u64>,
    r: Option<u64>,
    h: Option<u64>,
    w: Option<u64>,
}

impl Participant {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            a: None,
            e: None,
            r: None,
            h: None,
            w: None,
        }
    }

    fn generate_keys(&mut self) -> u64 {
        let mut rng = rand::thread_rng();

        let w = rng.gen_range(1..P - 1);
        self.w = Some(w);

        let h = mod_exp(G, w, P);
        self.h = Some(h);

        println!("{} generated h = {}", self.name, h);

        h
    }

    fn set_h(&mut self, h: u64) {
        self.h = Some(h);
    }

    fn get_commitment(&mut self) -> u64 {
        let mut rng = rand::thread_rng();

        let r = rng.gen_range(1..P - 1);
        self.r = Some(r);

        let commitment = mod_exp(G, r, P);

        println!("{} generated the commitment a = {}", self.name, commitment);

        commitment
    }

    fn receive_commitment(&mut self, commitment: u64) {
        self.a = Some(commitment);
    }

    fn generate_challenge(&mut self) -> u64 {
        let mut rng = rand::thread_rng();

        let challenge = rng.gen_range(1..P);
        self.e = Some(challenge);

        println!("{} generated the challenge e = {}", self.name, challenge);

        challenge
    }

    fn receive_challenge(&mut self, challenge: u64) {
        self.e = Some(challenge);
    }

    fn calculate_response(&self) -> u64 {
        let r = self.r.expect("Expected r while calculating response");
        let e = self.e.expect("Expected e while calculating response");
        let w = self.w.expect("Expected w while calculating response");

        let z = (r + e * w) % (P - 1);

        println!("{} generated the response z = {}", self.name, z);

        z
    }

    fn receive_response(&self, response: u64) {
        let a = self.a.expect("Expected a when receiving response");
        let e = self.e.expect("Expected e when receiving response");
        let h = self.h.expect("Expected h when receiving response");

        let left = mod_exp(G, response, P);

        println!(
            "{} found left-hand side:  {}^{} mod {} = {}",
            self.name, G, response, P, left
        );

        let right = (a * mod_exp(h, e, P)) % P;

        println!(
            "{} found right-hand side:  {} * {}^{} mod {} = {}",
            self.name, a, h, e, P, right
        );

        println!("{} checked {} = {}", self.name, left, right);

        if left == right {
            println!("Verification successful!");
        } else {
            println!("Verification failed!");
        }
    }
}

// Modular exponentiation
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
