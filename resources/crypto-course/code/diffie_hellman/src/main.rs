
const P: u32 = 11;
const G: u32 = 2;

fn main() {
    let mut alice = Participant {
        name: "Alice".to_string(),
        pk: None,
        sk: 3,
    };
    alice.generate_public_value();

    let mut bob = Participant {
        name: "Bob".to_string(),
        pk: None,
        sk: 5,
    };
    bob.generate_public_value();

    alice.generate_key(bob.get_public_value());
    bob.generate_key(alice.get_public_value());
}

struct Participant {
    name: String,
    pk: Option<u32>,
    sk: u32
}

impl Participant {
    fn generate_public_value(&mut self) {
        self.pk = Some(G.pow(self.sk) % P);

        println!("{} generated public key {}", self.name, self.sk);
    }

    fn get_public_value(&self) -> u32 {
        self.pk.expect(&format!("No public value found for {}", self.name).to_string())
    }

    fn generate_key(&self, public_value: u32) {
        let k = public_value.pow(self.sk) % P;

        println!("{} generated the key k = {}", self.name, k);
    }
}
