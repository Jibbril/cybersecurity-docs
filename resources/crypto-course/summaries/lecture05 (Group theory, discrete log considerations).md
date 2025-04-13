# Lecture 5

### Group theory
When introducing the concepts of public-key cryptography, one recurring topic is the idea of mathematical groups. A group $G$ is a finite set of element together with an operation $\oplus: G \times G \rightarrow G$ that has the following properties:

- **Closure**: The outcome of $\oplus$ will always be in $G$. So $g \oplus h \in G, \ \forall g,h \in G$.
- **Associativity**: $(g \oplus h) \oplus k = g \oplus (h \oplus k)$
- **Identity**: There exists an element $e \in G$ such that $e \oplus g = g \oplus e = g, \ \forall g \in G$
- **Inverse**: $\exists \overline{g} s.t.\ g \oplus \overline{g} = e$

### Important concepts for groups
#### Generators 
A generator is some $g \in G$ such that when the function $\oplus$ is applied to $g$ repeatedly, it ends up spanning the entire set. So, for example if we set the group $G$ to be the natural numbers modulo 11, and the function $\oplus$ to mean multiplication. We can then choose the number 2 and try applying $\oplus$ to it. 

```math
\begin{align*}
                            &  &g^0      &                   &                       &= 1 &\mod 11 \\
                            &  &g^1      &                   &                       &= 2 &\mod 11 \\
    g \oplus g              &= &g^2      &= 2 ^ 2            &= 2 \times 2           &= 4 &\mod 11 \\
    g \oplus g \oplus g     &= &g^3      &= g^2 \times g     &= 4 \times 2           &= 8 &\mod 11 \\
    g \oplus \dots \oplus g &= &g^4      &= g^2 \times g^2   &= 4 \times 4           &= 5 &\mod 11 \\
    \dots                   &= &g^5      &= g^4 \times g     &= 5 \times 2 \mod 11   &= 10 &\mod 11 \\
    \dots                   &= &g^6      &= g^4 \times g^2   &= 5 \times 4 \mod 11   &= 9 &\mod 11 \\
    \dots                   &= &g^7      &= g^6 \times g     &= 9 \times 2 \mod 11   &= 7 &\mod 11 \\
    \dots                   &= &g^8      &= g^6 \times g^2   &= 9 \times 4 \mod 11   &= 3 &\mod 11 \\
    \dots                   &= &g^9      &= g^8 \times g     &= 3 \times 2 \mod 11   &= 6 &\mod 11 \\
    \dots                   &= &g^{10}   &= g^8 \times g^2   &= 3 \times 4 \mod 11   &= 1 &\mod 11 \\
\end{align*}
```

As we see, 2 two is a generator because it spans the entirety of $G$ when combined iteratively with $\oplus$. Generators are typically denoted as $\langle g \rangle$.

#### Cyclic groups
If a group has at least one generator it is called a **cyclic group**.


#### Abelian groups
A group $(G,\oplus)$ is commutative (abelian) if $g \oplus h = h \oplus g \ \forall g,h \in G$.

#### Order of groups
The **order** of a group is the number of elements in it. $ord(G) = |G|$. However, the **order** of an element _in_ a group is the number of times $\oplus$ needs to be applied to itself to reach the unit number. So for some $h \in G$, how many times must we compute $h \oplus h$ until $h \oplus \dots \oplus h = e$

#### Facts about groups
- All cyclic groups are commutative
- The generator is never unique. If a group has one generator then it has several. 
- If a group has  order $n$ then all generators in the group also have order $n$ (since they need to touch each element once in their traversal). 
- For every $h \in G$ it holds that $ord(h)$ divides $ord(G)$.

### Euler's totient function
Euler's totient function $\phi : N \rightarrow N$ counts the number of positive integers smaller than a given argument that are relatively prime to it. So

```math
\phi(N) = N \prod_{p|N} \left( 1 - \frac{1}{p} \right)
```

One thing to note about Euler's totient function is that if $N$ is the product of two primes $p,q$ then the function simplifies to 

```math
\phi(N) = (p - 1)(q-1)

```

### Important results for groups
Below are some important and/or useful facts regarding groups.

- $`\forall a \in Z^*_N`$ it holds that $`a^{\phi(N)} = 1 \mod N`$
- If $p$ is prime, then $`\forall a \in Z^*_p, \ a^{p - 1} = 1 \mod p`$
- Let $N > 2$ be an integer: $`\forall a \in Z^*_N,\ x \in N`$, it holds that $`a^x \mod N = a^{x \mod \phi(x)} \mod N`$.

### Diffie-Hellman key exchange
The Diffie-Hellman key exchange was the first protocol allowing two users to share a combination of private and public values to independantly generate the same secret key. This key could then be used for further communication. The basic methodology goes as follows:

- Let $p$ be some large prime. 
- Find a generator $g$ of a subgroup of prime order $q$ in $`Z^*_P`$.
- Let $p,q,g$ be public information
- Let Alice and Bob generate some secret values $`a,b \leftarrow \$ Z_q`$. 
- Let Alice and Bob compute their public values $A,B$ as $`A = g^a \mod p`$ and $`B = g^b \mod p`$
- A shared secret key can now be computed by both Alice and bob using $`K = B^a = A^b`$.
- Commonly, this $K$ is hashed before usage

### Man-in-the-middle Attack against Diffie-Hellman
The basic implementation of the Diffie-Hellman key exchange has some severe problems. One of them is that it is very weak to man-in-the-middle attacks. In this scenario, Alice and Bob believe that they are communicating with each other, but unbeknownst to them there is an attacker in between them. So Alice is talking to the Adversary who in turn talks to Bob. When Alice sends her Public key to Bob, the attacker returns his own $`B^*`$ which is generated by his own $`a^*`$ rather than Bob's $B$. In this way, the attacker will form the same secret key as Alice the same way that Alice and Bob did in the previous example. Once The attacker performs the same procedure with Bob, he can fully monitor all the communication between Alice and Bob. 

### The Discrete Logarithm Assumption (DLA)
The discrete logarithm assumption is a core assumption with regard to DH-style cryptosystems. It is an assumption (cannot be proven) and states the following: 

Let $G$ be a cyclic group of order $q$ ($q$ is an $n$-bit long prime), and $g$ be a generator in $G$. The DLA states that it is computationally infeasible for any efficient attacker to find the exponent $x$ such that $`g^x = h`$ for some $h \in G$. Formally we have 

```math
Pr[x^* = x | x \leftarrow \$Z_q, x^* \leftarrow Attacker(q,g,g^x)] < negl(n)
```
### Three problems related to DLA
There are three problems related to the previous section, they are (in decreasing order of "hardness"):

- The discrete logarithm problem (DL): Given $A \in G$, find $a$ such that $`A = g^a`$
- The computational DH problem (CDH): Given $`A = g^a, B = g^b`$, find $`K = g^{a,b}`$
- The decisional DH problem (CDH): Given $`A = g^a`,\ B = g^b, \ C \in G`$, determine if $`C = g^{ab}`$ or not. 

### Examples of algebraic problems that are presumed to be "hard"
#### Integer factorization
Given some large $N$ that is the product of two primes $p,q$, it is computationally hard to find $p,q$ through factorization.

#### RSA assumption
The RSA protocol assumes that given $(N,e,y)$ s.t  
```math

\begin{align*}
    &N = p\times q \\
    &log_2(p) = log_2(q) = n \\ 
    &e \in Z^*_{\phi(N)} \\ 
    &y \in Z^*_N
\end{align*}
```

any PPT adversary has only negligible probability to find $`x \in Z^*_N`$ such that $`x^e = y \mod N`$.

#### Discrete Logarithm
Given some $`g,h \in Z_p`$, when $p$ is large (2048 bit), it is computationally hard to find $`x = DL_g(h)`$

### Trapdoor functions
A trapdoor function is a variation of a one-way function, but here it is very easy to invert the function if you know some secret, but very hard otherwise. All three problems in the previous section are prime candidates for this. It is very hard to perform integer factorization, but if you know $p$ or $q$ it is trivial even for large numbers. If you know $x$ then solving for the discrete logarithm is easy, and if you know $d$ then decrypting the RSA assumption is trivial. 
