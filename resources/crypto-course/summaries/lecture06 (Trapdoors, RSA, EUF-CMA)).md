# Lecture 6

### Examples of algebraic trapdoor functions
- **Integer factorisation**: $f(p,q) = N = p \cdot q$. Trapdoor: $(p,q)$
- **Discrete log**: $`f(x) = h = g^x \mod p`$. Trapdoor: $x$
- **RSA**: $`f_{N,e}(x) = y = x^e \mod N`$. Trapdoor: $d$, the inverse of $e \mod \varphi(x)$

### Trapdoor permutations
A (one-way) trapdoor permutation is a tuple $(KeyGen,F,I)$ where 

- $`KeyGen(1^n) \rightarrow (pk,sk)`$ is a probabilistic key generation algorithm outputting a public and a private key.
- $`F(pk,\cdot): X \rightarrow Y`$ is efficiently computable
- $`I(sk,\cdot): \rightarrow X`$ is efficiently computable
- $F$ is hard to invert without knowing $sk$

### RSA 
If we let $N$ be the product of two safe primes $p,q$, and $`e \in Z_N^*`$, then the function $`f_{N,e}(x) = x^e \mod N`$ is a trapdoor permutation over $`Z_N`$. 

This is given by the fact that $KeyGen$ outputs correct keys, $F$ and $I$ are efficiently computable, $I$ is the inverse of $F$, and given the RSA assumption (prime number factorization is computationally hard) it is hard to invert $F$ without the secret key. 

### Cryptographic signatures
A digital signature scheme is a triple $(KeyGen, Sign, Ver)$ where 
- $`KeyGen(1^n) \rightarrow (pk,sk)`$ is a probabilistic key generation algorithm outputting a public and a private key.
- $Sign(sk,m) \rightarrow \sigma$ is an algorithm that outputs a signature $\sigma$ for a message $m$.
- $`Ver(pk, m, \sigma) \in \{0,1\}`$ is a validation function that returns 1 for valid signatures $sigma$ and 0 otherwise.

### Existential Unforgeability Under Chosen Message Attack (EUF-CMA)
When talking about signatures, the adversary wants to successfully create a valid signature for a new message. To do this, he has the public key, as well as oracle access to the signer (who has the secret key). Given this, the existential unforgeability under chosen message attack (EUF-CMA) scenario is explained below. 

The challenger uses $KeyGen$ to generate a public and a secret key $(pk, sk)$. The adversary gains access to $pk$, and is able to send polynomially many adaptive queries to the challenger, who in turn will return the signature for the sent message. Once the adversary is content, he can send a tuple $`(m^*,\sigma^*)`$ to the challenger. If $`m^*`$ is a new message, and the challenger verifies $`\sigma^*`$ as a proper signature, the adversary wins.

A digital signature scheme is said to be secure if the probability of the adversary winning the EUF-CMA security game is negligible. 

### RSA and EUF-CMA
It is worth noting that the textbook RSA implementation does not fulfil the demands of EUF-CMA. This has to do with the mathematical properties of RSA, in particular multiplication under modulo. If the adversary sends a message $`m_0`$ and is returned $`\sigma_0`$, and then does the same for $`m_1,\sigma_1`$. He can then find a correct verification by setting 

```math
\begin{align*}
    m^* &= m_0 \cdot m_1 \mod N \\
    \sigma^* &= \sigma_0 \cdot \sigma_1 \mod N
\end{align*}
```

However, there are variations on the standard RSA scheme that solve this problem. To make RSA EUF-CMA secure we have RSA-FDHS, and to make RSA IND-CCA safe, use RSA-OAEP. 

### The Hash and Sign Paradigm  (Full Domain Hash Signatures)
One way of conceptualising hash signatures is by using $F$ and $I$ in reverse order. So consider a $Sign$ function that uses a message $m$ and the secret key $sk$ to compute a signature $\sigma = I(sk,H(m))$ where $H$ is some hash function. A verifier $Ver$ could then be built by computing $F(pk,\sigma) = H(m)$. In this way, a sender can encrypt a message using their own private key, and everyone else can verify it using the corresponding public key. Given this scenario, the following theorem is true (although not proven in this course):

If $`(KeyGen_{TD},F,I)`$ is a one-way trapdoor function and $H$ is a random oracle, then FDHS is a chosen message attack secure digital signature scheme. 


