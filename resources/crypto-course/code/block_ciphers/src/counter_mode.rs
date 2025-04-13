pub fn run_example() {
    println!("===============================================");
    println!("========= Starting Counter example ============");
    println!("===============================================");
    
    let alice = Participant::new("Alice");
    let bob = Participant::new("Bob");
    let eve = Participant::new("Eve");
    
    // In reality these shared parameters would be much longer, but they are 
    // short here for simplicity
    let key = 0xAB; 
    let nonce = 1;

    let message_to_send = "My super secret counter message!"; 

    let ciphertext = alice.encrypt_msg(message_to_send, nonce, key);
    eve.eavesdrop(&ciphertext);
    bob.decrypt_msg(&ciphertext, nonce, key);
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

    fn encrypt_msg(&self, msg: &str, nonce: u8, key: u8) -> Vec<u8> {
        // Convert string to bytes
        let message_bytes: Vec<u8> = msg.bytes().collect();
        
        // Encrypt message
        let encrypted_msg = Participant::process_msg(&message_bytes, nonce, key);

        println!("{} encrypted the message '{}' as the ciphertext: {:?}", self.name, msg, encrypted_msg);
        
        encrypted_msg
    }

    fn decrypt_msg(&self, ciphertext: &[u8], nonce: u8, key: u8) {
        println!("{} received the ciphertext: {:?}", self.name, ciphertext);

        // Note that encryption and decryption is the same process
        let msg_bytes = Participant::process_msg(ciphertext, nonce, key);
        let msg = String::from_utf8(msg_bytes).expect("Expected successful decryption.");
        
        
        println!("{} decrypted the ciphertext to: {:?}", self.name, msg);
    }

    fn eavesdrop(&self, ciphertext: &[u8]) {
        println!("{} eavesdropped ciphertext: {:?}", self.name, ciphertext);
    }

    fn process_msg(bytes: &[u8], nonce: u8, key: u8) -> Vec<u8> {
        let mut ciphertext = vec![];
        let mut counter = 0;

        for plaintext_block in bytes {
            let count: u8 = nonce + counter;

            // Normally this would be a much more complex encryption like AES 
            // or similar. Here we only XOR for demonstration purposes.
            let block_cipher = count ^ key as u8;

            // Encrypt the plaintext block with the cipher block
            let encrypted_block = plaintext_block ^ block_cipher;

            ciphertext.push(encrypted_block);

            counter += 1;
        }

        ciphertext
    }
}

