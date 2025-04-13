# Lecture 10: Cryptography and Privacy

### Attribute Based Credentials (ABC)
The key idea behind Attribute Based Credentials is to enable authentication of different attributes of an entity without revealing additional information about the entity. An example of a problem scenario could be identifying yourself with your driver's license when buying alchohol. The cashier only needs to know that you are over 18, not your exact birthdate or which vehicles you are allowed to drive.

ABC depends on the following cryptographic building blocks: 
- Generalised Pedersen commitments
- Cryptographic signatures
- Group signatures
- Zero-knowledge proofs
- Blind signatures

### Group Signatures
A group signature is a cryptographic construction where members of a group can cryptographically sign messages on behalf of a group (not themselves). Commonly, the members of the group will each have a private key. Also, the group itself will have a public key. These two can then be combined in order to let group members sign on behalf of the group. Additionally, there will be a group manager selected who has the ability to determine which member signed on behalf of the group, but from the outside it is not possible to determine this.

### Zero-knowledge proofs (intuition)
In zero-knowledge proofs (ZKP) one generally considers a Prover and a Verifyer. The prover wants to prove to the verifyer that some statement is true, without conveying any other information. 

An ZKP needs to be 
- __Complete__: If the prover knows $x$, they can convince the verifier
- __Sound__: If the prover doesn't know $x$, they can't convince the verifier
- __Zero knowledge__: The verifier doesn't learn any other information than that the prover knows $x$

### Blind Signatures (intuition)
A blind signature scheme is a signature  scheme where the signing algorithm is replaced by an interactive protocol run between a signer and a receiver. Although not described in detail here, the interaction will result in the receiver obtaining a signature $\sigma$ on a message $m$, while the sender learns nothing about $m$ nor $\sigma$.

A practical example is given by a scenario where a bank wants to sign some eCoin. In this scenario, Alice begins by taking the amount of money $x$ that she wants to spend, hashing it and multiplying it with some random number only she knows. She gets 
```math
\begin{equation*}
    B = r^e H(x) \mod n
\end{equation*}
```
She then sends $B$ to the bank which in turn signs the amount by taking
```math
\begin{equation*}
    \bar{S} = B^d \mod n
\end{equation*}
```
where $d$ is some secret key. Once the bank returns $\bar{S}$, Alice can extract her signature by using the multiplicative inverse of the randomness $r$ that only she knows.
```math
\begin{equation*}
    S = r^{-1}\bar{S} \mod n
\end{equation*}
```
$S$ is now a valid signature, even though the bank has never seen $x$ or $H(x)$. Alice can now pass the tuple $(x,S)$ to Bob who can in his turn pass it forward to the bank for verification. 


