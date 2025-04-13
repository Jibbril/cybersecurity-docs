# Lecture 2

### Pseudo Random Generators (PRG)
A PRG is a deterministic function $`PRG: \{0,1\}^n -> \{0,1\}^l`$ such that:
- $PRG(\cdot)$ is efficiently computable
- $l > n$
- $`\{PRG(x) | x \leftarrow \$ \{0,1\}^n\}`$ is pseudo-random (i.e. an efficient adversary cannot separate $PRG(x)$ from any $`y \leftarrow \$ \{0,1\}^l`$

There are no mathematical proofs to verify a PRG, only tests and formal security games. These games generally consist of some scenario where a challenger will either generate a key from the "real" distribution $`k \leftarrow \{0,1\}^l`$, or generate a key by using $PRG(\cdot)$. The adversary then needs to try and guess whether the provided key was from the real distribution or from the PRG. If the adversary can consistently guess correctly then the candidate algorithm is not a PRG. More formally we can say that a PRG is secure if 

$$
Adv = |Pr[Adversary\ wins] - \frac{1}{2} | < negl(n)
$$

### The negligible function
The negligible function $negl(x): \mathbb{N} \rightarrow \mathbb{R}_{\geq 0}$ has the key property that that for any positive polynomial $p(x)$ we have

$$
negl(x) \leq \frac{1}{p(x)} \quad \quad \forall x \geq x_{0} \in \mathbb{N}
$$

So essentially, $negl(x)$ will go to 0 faster than one divided by any polynomial function after some specific $x_0$. 

### Semantic security
Having established PRGs, we can now reason about scenarios where Shannon's theorem doesn't hold ($|K| \leq |C|$). For this we will define Semantic Security.

A symmetric encryption scheme is semantically secure if any efficient attacker $Adv$ has only negligible advantage in winning the semantic security game. Formally:

$$
Adv = |Pr[Adversary\ wins] - \frac{1}{2} | < negl(n)
$$

We can also look at this process through a **semantic security game**. In this game, an adversary generates two messages $m_{0},m_{1}$ and sends them to the challenger. The challenger generates a key, flips a bit $b$, encrypts either $m_{0}$ or $m_{1}$ according to $b$, and returns the ciphertext to the adversary. The adversary then uses $m_{0},m_{1}$ and $c$ to try and determine which message was encrypted. If the adversary can guess b correctly he wins. 

### Pseudo Random Permutations (PRP)
A pseudorandom permutation sounds similar to a PRG, but it actually does something very different. Where the PRG takes an existing key and expands it in a way that appears random, a PRP takes an input of a specific length and shuffles it into a different permutation in a way that also appears random. However, given that a receiver has a correct key, they can actually "un-shuffle" this permutation into the original message. So it's sort of like if you put fruit into the blender and start, it will get sliced. But if you could play back time you could go back from sliced to whole. 

Formally, a PRP is a function $`f_{k}:\ \{0,1\}^n \rightarrow \{0,1\}^n`$. With this, encryption is defined as 

$$
c = (s \parallel f_k(s) \oplus m)
$$

for a random s that is different for every message.

### Block ciphers
A block cipher is a function that builds on the concept of PRPs. Given some key space $K$ and an block space $M$, a block cipher is a deterministic and invertible function such that given some key $k$, the functions 

```math
\begin{align*}
E(k,\cdot):\ \{0,1\}^n \rightarrow \{0,1\}^n \\
D(k,\cdot):\ \{0,1\}^n \rightarrow \{0,1\}^n
\end{align*}
```

exist and have the property that 

$$
D(k, E(k,m)) = m, \quad \forall m \in \{0,1}^n
$$

Or in other words: $E$ is invertible knowing the key $k$.

A key point of this system is that now the same key $k$ can be used to encrypt/decrypt multiple messages (as compared to OTP where each key could only be used once). This is due to the pseudo random nature of $E$ giving that an efficient adversary cannot find the original message without having access to the key $k$ used to run $E(k,\cdot)$.

### Chosen paintext attack (security notion for block ciphers)
The aim for the chosen plaintext attack is to quantify an adversarys likelihood of breaking a block cipher. It starts by allowing an adversary to send polynomially many (think $2^{60}$) messages to the challenger and receiving them back encrypted using the same key. The adversary is then allowed to send a tuple of two new messages of the same length from which the challenger will flip a bit $b$ and randomly encrypt the corresponding message and return it to the adversary. If the adversary can successfully guess which message was encrypted he wins. 

### Feistel networks
A recurring idea when using block ciphers is to recursively shuffle the message that is to be encrypted. In feistel networks, the cipher function $F$ is the same for every round, and it does not need to be invertible for the entire round to be invertible. However, decryption is equal to encryption, but with the round-keys in reverse order.

### Operation modes for block ciphers
**Note that there are code examples available in the code folder for all modes below.**

#### Electronic Code Book Mode (ECB)
In this mode, each block gets encrypted separately using the same key. The benefits of this is that it is easy to understand and implement as well as highly parallelizable which increases speed. However, it is not recommended for real world use in cryptographic protocols as there are serious weaknesses.

One such weakness can be illustrated by a mathematical game. Imagine an adversary who sends two messages $(m_0,m_1)$ to a challenger. The challenger generates a key and flips a bit $b$. Depending on $b$ the challenger will use AES-ECB to encrypt $m_b$ and send the resulting ciphertext to the adversary. The adversary must then guess for $b$. 

In this scenario, if the adversary sends 

$$
\begin{align*}
    &m_0 = (block0,block1) \\
    &m_1 = (block1,block1) \\
\end{align*}
$$

they can simply look at the returned $c = (c_0,c_1)$ to determine $b$. If $c_0 = c_1$ then $b=1$, else $b = 0$.

#### Cipher Block Chaining Mode (CBC)
In this mode, in addition to the regular encryption, the output from the previous block is XORed with the second message before encryption. For the initial block an initialization vector for the first XOR is required. In this way a link is created between the blocks which avoids many of the problems related to the ECB mode. However, there are some drawbacks. For example, encryption is sequential which reduces speed.

#### Counter mode 
Counter mode is a bit more involved than the previous two, but the general idea is similar to CBC. However, instead of using the previous ciphertext to XOR, we here introduce a nonce that gets incremented for each block that is encrypted. 

The process begins by the two parties somehow sharing a key and a beginning nonce. The sending party then encrypts their message into blocks by incrementing the nonce by one and using some cryptographic procedure (commonly AES or similar) to combine the key with the incremented nonce. For illustration here (and in the block cipher code example) the procedure can be set to simply the XOR operation itself. Next, the plaintext is XORed with the result of the previous encryption to produce the first encrypted block. The process is then repeated for the next plaintext block, but with the counter incremented once again. 

One neat aspect of this mode is that encryption and decryption are the exact same process. This is due to the XOR property that if a binary string is XORed with the same string twice, it returns the original string. It is however important to note that it is extremely important to only use each nonce once, otherwise there will be information leakage.
