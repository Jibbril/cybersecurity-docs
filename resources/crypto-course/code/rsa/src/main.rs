/// Simple illustration of the RSA protocol where a participant Alice wants
/// to send a message to another participant named Bob.
fn main() {
    // Initialize participants
    let mut bob = Participant::init_bob();
    let alice = Participant::init_alice();

    // Take steps to prepare a public and private key for Bob
    bob.generate_n();
    bob.generate_totient();
    bob.find_e();
    bob.generate_d();
    bob.print_keys();

    // Encrypt message that Alice wants to send to Bob
    let message = 4;
    let bob_pkey = bob.get_public_key();
    let ciphertext = alice.send_encrypted_message(bob_pkey, message);

    // Create eavesdropping adversary
    let eve = Participant::init_eve();
    eve.eavesdrop(ciphertext);

    // Bob receives and decrypts message
    bob.receive_message(ciphertext);
}

struct Participant {
    name: String,
    p: i64,
    q: i64,
    n: Option<i64>,
    e: Option<i64>,
    d: Option<i64>,
    totient: Option<i64>,
}

impl Participant {
    fn init_bob() -> Self {
        Participant {
            name: "Bob".to_string(),
            p: 3, // Prime chosen by Bob, much larger in reality
            q: 11, // Prime chosen by Bob, much larger in reality
            n: None, // To be generated
            e: None, // To be generated
            d: None, // To be generated
            totient: None, // To be generated
        }
    }

    fn init_alice() -> Self {
        Participant {
            name: "Alice".to_string(),
            p: 5, // Prime chosen by Alice, much larger in reality
            q: 13, // Prime chosen by Alice, much larger in reality
            n: None, 
            e: None,
            d: None, 
            totient: None, 
        }
    }

    fn init_eve() -> Self {
        Participant {
            name: "Eve".to_string(),
            p: 1,
            q: 1,
            n: None,
            e: None,
            d: None,
            totient: None,
        }
    }

    fn generate_n(&mut self) {
        let n = self.p * self.q;
        self.n = Some(n);

        println!("{} generated n: {}", self.name, n);
    }

    fn generate_totient(&mut self) {
        let totient = (self.p - 1) * (self.q - 1);
        self.totient = Some(totient);

        println!("{} generated totient: {}", self.name, totient);
    }

    fn find_e(&mut self) {
        let mut candidate = 2; // Start from the lowest prime other than 1
        let totient = self.totient.expect("Expected totient");

        loop {
            let (gcd,_,_) = extended_gcd(totient, candidate);

            if gcd == 1 || candidate > 100 {
                break
            }

            candidate += 1;
        }

        println!("{} generated e: {}", self.name, candidate);

        self.e = Some(candidate);
    }

    fn generate_d(&mut self) {
        let e = self.e.expect("expected e");
        let totient = self.totient.expect("expected totient");

        let (gcd, x, _) = extended_gcd(e, totient);

        if gcd == 1 {
            let d = (x % totient + totient) % totient;

            println!("{} generated d: {}", self.name, d);

            self.d = Some(d);
        } else {
            panic!("Unable to generate d.");
        }
    }
    fn print_keys(&self) {
        let n = self.n.expect("Expected n");
        let e = self.e.expect("Expected e");
        let d = self.d.expect("Expected d");

        println!("{} has public key: ({},{})", self.name, n, e);
        println!("{} has private key: ({},{})", self.name, n, d);
    }

    fn get_public_key(&self) -> (i64, i64) {
        let n = self.n.expect("Expected n");
        let e = self.e.expect("Expected e");

        (n,e)
    }

    fn send_encrypted_message(&self, pkey: (i64,i64), message: i64) -> i64 {
        let (n,e) = pkey;
        let c = message.pow(e as u32) % n;

        println!("{} encrypted message {} to ciphertext {} using public key {:?}", self.name, message, c, pkey);

        c
    }

    fn eavesdrop(&self, ciphertext: i64) {
        println!("{} eavesdropped the ciphertext: {}",self.name, ciphertext);
    }

    fn receive_message(&self, ciphertext: i64) {
        let n = self.n.expect("Expected n");
        let d = self.d.expect("Expected d");

        let decrypted_message = ciphertext.pow(d as u32) % n;

        println!("{} received cipher {} and decrypted it to: {}", self.name, ciphertext, decrypted_message);
    }
}

/// Implementation of the extended euclidean algorithm to find d
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (x, y, z) = extended_gcd(b, a % b);

        (x, z, y - (a / b) * z)
    }
}

