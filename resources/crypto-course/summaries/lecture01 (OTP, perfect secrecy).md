# Lecture 1

### Kerckhoff's principle (1884)
A cryptographic system should be secure even if everything about the system, except for the key, is public knowledge.

### Terminology and notation
- Notation for drawing randomly from a distribution utilizes the arrow as in $`b \leftarrow \$ \{0,1\}`$. The \$ signifies that it is a uniform sample from the set $`\{0,1\}`$.
- Conditional probabilities are very common. $Pr[A|B]$ implies "Probability of $A$ given $B$".
- Adding an exponent to a sample means sampling that number. So $`k \leftarrow \{0,1\}^n`$ would imply that $k$ is a binary string of length $n$.

### Symmetric encryption
Symmetric encryption is defined as the tuple $(KeyGen,E,D)$ over the key space $K$, message space $M$, and ciphertext space $C$. 

- $KeyGen(1^n) \rightarrow k$ is a randomized algorithm that returns a key.
- $E(k,m)$ represents the encryption of a message $m$ using key $k$
- $D(k,m)$ represents the decryption of a message $m$ using key $k$

### One Time Pad (OTP)
One time pad is an example of symmetric encryption where the encryption consists of using the logical XOR operation on a messages and a key of the same length. So 

$$
\begin{align*}
    m: &0110 \\
    k: &1100
\end{align*}
$$

Would give ciphertext 

$$
c: 1010
$$

### Perfect secrecy
A symmetric encryption scheme $(KeyGen,E,D)$ is perfectly secret if the following holds for all $m_0, m_1$ in $M$:

$$
Pr[c \leftarrow E(k, m_0) | k \leftarrow KeyGen(1^n)] = Pr[c \leftarrow E(k, m_1) | k \leftarrow KeyGen(1^n)]
$$

So, there must be an equal probability for each message to be encrypted into a specific ciphertext. Or the other way around: it must be impossible to know which was the original message for a given ciphertext. 

### Proof of OTP security
We begin by showing that OTP is a symmetric encryption scheme. Consider 

$$
\begin{align*}
    &K = M = C = $\{0,1\}^n \\
    &KeyGen(1^n): k \leftarrow \$ \{0,1\}^n \\
    &E(k,m) = m \oplus k \\
    &D(k,c) = c \oplus k
\end{align*}
$$

where $\oplus$ represents the XOR operation. We then have correctness (no chance that the message can be corrupted/changed in the process of encryption or decryption) due to the XOR property

$$
D(k, E(k,m)) = k \oplus c = k \oplus (k \oplus m) = m
$$

Next we consider that there is exactly one key that will encrypt a given message to a specific ciphertext. So 

$$
Pr[c \leftarrow E(k,m) | k \leftarrow KeyGen(1^n)] = Pr[c \oplus m \leftarrow KeyGen(1^n)] = \frac{1}{|K|}
$$

This is true for all messages, and thus we have that

$$
Pr[c \leftarrow E(k, m_0) | k \leftarrow KeyGen(1^n)] = Pr[c \leftarrow E(k, m_1) | k \leftarrow KeyGen(1^n)] = \frac{1}{|K|}
$$

which is exactly the definition of perfect secrecy.

### Problems of OTP
1. Key must be as long as the message
2. Key can only be used to encrypt one message ($c_0 \oplus c_1 = (k \oplus m_0) \oplus (k \oplus m_1) = m_0 \oplus m_1$)
3. The ciphertext is malleable

### Shannon's theorem (1940)
If a symmetric encryption scheme (KeyGen,E,D) defined over (K,M,C) has perfect secrecy then $|K| \geq |M|$.

The proof of this ties to the fact that only one key can generate any one ciphertext. If any one key could generate two different ciphertexts from the same message we would instantly lose correctness since decryption would become ambigous. As such, we know that for each message there must exist at least one key, which means that the total number of keys must be larger or equal to the total number of messages, meaning that $|K| >= |M|$.

### Pseudorandom Generators (PRGs)
One of the key takeaways from Shannon's theorem is that perfect secrecy is not very practical due to the large number of keys and ciphertexts needed. To lessen this burden we introduce the concept of Pseudorandom Generators (PRGs). The main idea of a PRG is that it is a function that takes in some shorter key and expands it to some longer length in such a way that an adversary cannot determine whether the end product has been sampled directly from the relevant distribution. More formally:

A PRG is a deterministic function $`PRG: \{0,1\}^n \rightarrow \{0,1\}^l`$ such that $PRG(\cdot)$ is efficiently computable, $l > n$ and the outcome of running $PRG$ is pseudo-random (no adversary can tell the difference between $`PRG(x \leftarrow \$ \{0,1\}^n)`$ and $`y \leftarrow \$ \{0,1\}^l`$. 

There is no mathematical way to prove that a candidate algorithm is a PRG, we can only test extensively and try to see if it works.
