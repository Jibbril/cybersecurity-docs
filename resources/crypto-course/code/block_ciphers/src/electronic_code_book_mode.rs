pub fn run_example() {
    println!("===============================================");
    println!("=========== Starting ECB example ==============");
    println!("===============================================");
    
    let alice = Participant::new("Alice");
    let bob = Participant::new("Bob");
    let eve = Participant::new("Eve");
    
    // In reality this key would be much longer
    let key = 0xAB; 

    let message_to_send = "My super secret ECB message!"; 

    let ciphertext = alice.encrypt_msg(message_to_send, key);
    eve.eavesdrop(&ciphertext);
    bob.decrypt_msg(&ciphertext, key);
}

struct Participant {
    name: String
}

impl Participant {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }

    fn encrypt_msg(&self, msg: &str, key: u8) -> Vec<u8> {
        // Convert string to bytes
        let message_bytes: Vec<u8> = msg.bytes().collect();
        
        // Encrypt message
        let mut ciphertext = vec![];

        for plaintext_block in message_bytes {
            let cipher: u8 = plaintext_block ^ key;
            ciphertext.push(cipher);
        }

        println!("{} encrypted the message '{}' as the ciphertext: {:?}", self.name, msg, ciphertext);
        
        ciphertext
    }

    fn decrypt_msg(&self, ciphertext: &[u8], key: u8) {
        println!("{} received the ciphertext: {:?}", self.name, ciphertext);

        let mut plaintext_bytes = vec![];

        for block in ciphertext {
            let decrypted_block = block ^ key;
            plaintext_bytes.push(decrypted_block);
        }

        let msg = String::from_utf8(plaintext_bytes).expect("Expected successful decryption.");
        
        println!("{} decrypted the ciphertext to: {:?}", self.name, msg);
    }

    fn eavesdrop(&self, ciphertext: &[u8]) {
        println!("{} eavesdropped ciphertext: {:?}", self.name, ciphertext);
    }
}

