pub fn run_example() {
    println!("===============================================");
    println!("=========== Starting CBC example ==============");
    println!("===============================================");
    
    let alice = Participant::new("Alice");
    let bob = Participant::new("Bob");
    let eve = Participant::new("Eve");
    
    // In reality this key would be much longer
    let key = 0xAB; 
    let initialization_vector = 0x00;

    let message_to_send = "My super secret CBC message!"; 

    let ciphertext = alice.encrypt_msg(message_to_send, key, initialization_vector);
    eve.eavesdrop(&ciphertext);
    bob.decrypt_msg(&ciphertext, key, initialization_vector);
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

    fn encrypt_msg(&self, msg: &str, key: u8, iv: u8) -> Vec<u8> {
        // Convert string to bytes
        let message_bytes: Vec<u8> = msg.bytes().collect();
        
        // Encrypt message
        let mut ciphertext = vec![];
        let mut prev_block = iv;

        for plaintext_block in message_bytes {
            // xor plaintext blow with previous block
            let xored_block = plaintext_block ^ prev_block;

            // xor result with key
            let encrypted_block: u8 = xored_block ^ key;

            ciphertext.push(encrypted_block);

            prev_block = encrypted_block;
        }

        println!("{} encrypted the message '{}' as the ciphertext: {:?}", self.name, msg, ciphertext);
        
        ciphertext
    }

    fn decrypt_msg(&self, ciphertext: &[u8], key: u8, iv: u8) {
        println!("{} received the ciphertext: {:?}", self.name, ciphertext);

        let mut plaintext_bytes = vec![];
        let mut prev_block = iv;

        for block in ciphertext {
            let decrypted_block = block ^ key;
            let original_block = decrypted_block ^ prev_block;

            plaintext_bytes.push(original_block);
            prev_block = *block;
        }

        let msg = String::from_utf8(plaintext_bytes).expect("Expected successful decryption.");
        
        println!("{} decrypted the ciphertext to: {:?}", self.name, msg);
    }

    fn eavesdrop(&self, ciphertext: &[u8]) {
        println!("{} eavesdropped ciphertext: {:?}", self.name, ciphertext);
    }
}

