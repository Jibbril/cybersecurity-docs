/// A simple illustration of the One Time Pad cryptographic system
fn main() {
    // binary 1100 as key that is shared between Alice and Bob
    let key: u8 = 0b1100;

    // binary 0110 as the secret message
    let message: u8 = 0b0110;

    let alice = Participant {
        name: "Alice".to_string(),
        key: Some(key),
        message: Some(message),
    };

    let bob = Participant {
        name: "Bob".to_string(),
        key: Some(key),
        message: None,
    };

    let eve = Participant {
        name: "Eve".to_string(),
        key: None,
        message: None,
    };

    // Alice encrypts and sends the message
    let sent_ciphertext = alice.encrypt_message();

    // Eve eavesdrops the ciphertext but cannot read it since she does not have
    // the key to decrypt
    eve.eavesdrop(sent_ciphertext);

    // Bob has the key and can thus decrypt
    bob.decrypt_message(sent_ciphertext);
}

struct Participant {
    name: String,
    key: Option<u8>,
    message: Option<u8>,
}

impl Participant {
    fn encrypt_message(&self) -> u8 {
        let message = self.message.expect("Expected message");
        let key = self.key.expect("Expected key");

        let ciphertext = message ^ key;
        println!(
            "{} encrypted the message {:04b} into the ciphertext {:04b}.",
            self.name, message, ciphertext
        );

        ciphertext
    }

    fn decrypt_message(&self, ciphertext: u8) {
        let key = self.key.expect("Expected key");
        let message = ciphertext ^ key;

        println!(
            "{} received {:04b} and decrypted it to {:04b}!",
            self.name, ciphertext, message
        );
    }

    fn eavesdrop(&self, message: u8) {
        println!("{} eavesdropped the message: {:04b}", self.name, message);
    }
}
