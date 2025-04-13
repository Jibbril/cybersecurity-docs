# Lecture 7

### Number of keys needed in a network
When using symmetric encryption in a network, the total amount of keys needed for everyone to be able to communicate with everyone is $`\frac{n (n - 1)}{2}`$. For public-key encryption this number falls to only $n$. 

### Public-Key Encryption (PKE)
A Public-Key encryption (PKE) is given by the triple $(KeyGen,Enc,Dec)$ where 

- $`(pk,sk) \leftarrow KeyGen(1^n)`$ produces a public and a private key
- $`c \leftarrow Enc(pk, m)`$ is encryption for some $m \in M$
- $`m' = Dec(sk,c)`$ is decryption.
- $m = m'$

### Encryption examples
Examples of Diffie-Hellman, Elgamal and RSA are available in the code examples. 

### Properties of encryption schemes
|           |Textbook RSA   | Elgamal   | RSA OAEP
|-----------|---------------|-----------|-----------|
|Secure TDF?| yes*          | no        | no        |
|Malleable? | yes           | yes       | no        |
|IND-CPA?   | no            | yes*      | yes*      |
|IND-CCA?   | no            | no        | yes*      |

$*$ under the corresponding computational assumptions

### Homomorphic encryption
There are two versions of homomorphic encryption, linear and full.

- Linearly homomorphic encryption can only add and subtract
- Fully homomorphic encryption can also multiply.

### Linear homomorphic encryption 
Linear homomorphic encryption (LHE) is a PKE triple $(KeyGen, Enc, Dec)$ with an associated algorithm $`\oplus_{pk}`$ such that 

```math
Enc_{pk}(a) \oplus_{pk} Enc_{pk}(b) = Enc_{pk}(a \star b)
```

Where $\star$ is some operation. It is worth pointing out that the secret key $sk$ is not used anywhere in this process. 

### Elgamal LHE
Assume that we have two messages $`m_1, m_2`$ that are encrypted using a key $A$ into $`(B_1, c_1), (B_2, c_2)`$. If Alice wants to decrypt the result of applying the LHE operation using her private key $a$, we get 

```math
\begin{align*}
    K &= (B_1 \cdot B_2)^a = B_1^a \cdot B_2^a \\
    m &= c_1 c_2 \cdot K^{-1} \mod p \\
      &= (c_1 B_1^{-a})(c_2 B_2^{-a}) \mod p \\
      &= m_1 m_2 \mod p
\end{align*}
```

### RSA LHE 
Assume that we have two RSA ciphertexts $`m_1^e, m_2^e`$ going to an Alice with private key $(N,d)$. If she decrypts the messages after applying LHE we get 

```math
\begin{align*}
    m &= (m_1^e m_2^e)^d \mod N \\
      &= (m_1 \cdot m_2)^{ed} \mod N \\
      &= m_1 m_2 \mod N
\end{align*}
```

### Fully Homomorphic Encryption (FHE)
Fully Homomorphic Encryption (FHE) is the same as LHE with the addition of an operator $\odot$ such that 

```math
\begin{equation*}
    Enc_{pk}(a) \odot_{pk} Enc_{pk}(b) = Enc_{pk}(a \star a \star \dots \star a)
\end{equation*}
```

where $\star$ is applied $b$ times. 


