# Lecture 4

### Security of different block cipher modes
When it comes to block cipher modes, CBC and CTR modes are secure if properly implemented while ECB is not. 

### Indistinguishability under chosen plaintext attack (IND-CPA)
IND-CPA is a standard way of evaluating/testing a security framework. In this scenario, the adversery's goal is to distinguish between the encryption of two known plaintext messages. To do this, the adversary can send polynomially many adaptive messages to the challenger and receive the corresponding ciphertexts. Once content, the adversary then sends two messages of which the challenger randomly encrypts and returns one. If the adversary can determine which message was selected for encryption, he wins. 

### Padding
When sending block ciphers there can sometimes be a need to fill the last (or some intermediate) block with padding to ensure proper bit length. This is quite perilous since poorly implemented padding can be a serious attack vector. Below are some key points for proper padding

- Padding must be reversible, the receiver needs to be able to safely remove the padding.
- An example of insecure padding is to simply add the required number of bits as zeros, this can easily cause patterns that can be broken.
- The receiver should always check that the padding was applied correctly before removing it. 

### Message Authentication Codes (MAC)
Proper encryption guarantees that no information will be leaked through the ciphertext, however, it does not guarantee that the ciphertext itself has not been tampered with or modified. To fix this, we need to introduce the concept of authenticity or integrity, meaning essentially that the message has not been changed. A common way of achieving this is by using MACs which are defined as follows:

A MAC is a pair of efficient algorithms $MAC,Ver$ with the properties

- $`MAC: K \times M \rightarrow T`$ is a probabalistic algorithm
- $`Ver: K \times M \times T \rightarrow \{0,1\}`$ is a deterministic algorithm that accepts ($1$) or rejects ($0$) depending on its input.
- $MAC$ and $Ver$ satisfy that $`Pr[Ver(k,m,MAC(k,m)) = 1] = 1 \quad \forall k \in K, m \in M`$

### Unforgeability under Chosen Message Attack
Given the structure of MACs, we also want some way of reasoning about the reliability of them. For this, we introduce the concept of Unforgeability under chosen message attack. The goal of the adversary here is to produce a tag $t$ for some known message that will make the receiver judge the modified messages as valid, even though it is different from the actual message sent. To manage this, the adversary is now allowed to drop, replace and inject information into the communication channel. The adversary also has oracle access to both $MAC$ and $Ver$ to test them polynomially many times. Given this, the adversary wins if he can send a tuple $`(m^*,t^*)`$ which is successfully verified by $Ver$. 

In this scenario, a MAC is said to be secure if 

```math
Pr[Ver(k, m^*,t^*) = 1 | (m^*,t^*) \leftarrow Adversary, m^* \ \text{is new}] \leq negl(n)
```

### CBC-MAC
When sending multiple blocks, they can be authenticated using the same tag by chaining the tags together with each block, similar to the CBC block cipher mode. The process starts by running the first message throught the $MAC$ function to output a tag. This tag would then be XORed with the next message before encryption. This chain continues until we arrive at the final tag which will consequently then be linked to all the previous blocks. However, to avoid RAW CBC-MAC message extension attacks, we also need to XOR with a final random key before the last block (ANSI CBC-MAC). Otherwise there are ways of unwinding the tag-chain that enable invalidation of the MAC. 

### Authenticated encryption
Having achieved security notions for both encryption and authenticity, we can now combine them to achieve both for the same message. It is however important to note that the order of this combination matters. If we encrypt and MAC a message we may still be open to attack, so the proper way is to first encrypt, then create the MAC from the encrypted ciphertext (encrypt-then-mac). One very popular implementation of this sort of system is called Galois Counter Mode (GCM). 
