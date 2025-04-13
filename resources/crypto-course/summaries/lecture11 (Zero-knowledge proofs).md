# Lecture 11: Zero-knowledge Proofs

### $\Sigma$-Protocols
When talking about zero-knowledge proofs and $\Sigma$-protocols in particular, we consider a scenario where two actors (Prover and Verifier) send the following three messages back and forth:
- The Prover sends a value $a$ to the Verifier
- The Verifier returns a value $e$ to the Prover
- The Prover sends a value $z$ back to the Verifier who either accepts or rejects

The protocol satisfies the following three properties: 
- __Completeness__: If P and V follow the protocol on some input $x$ and private input $w$ where $(x,w) \in R$, then V always accepts.
- __Special Soundness__: There exists some PPT algorithm $E$ (extractor) that for any pair of accepting transcripts $(a,e,z)$, $(a,e',z')$ for $x$ with $e \neq e'$, outputs $w$ s.t. $(x,w) \in R$.
- __Special honest verifier zero knowledge (SHVZK)__: For every $x$ and $w$ s.t. $(x,w) \in R$ and every $`e \in \{0,1\}^t`$ here exists a PPT algorithm $Sim$ (simulator) which given $(x,e)$ outputs transcripts $(a,e,z)$ that are distributed like real conversations. That is, $`\{Sim(x,e)\} =_c \{\langle P(x,w), V(x,e) \rangle \}`$

Intuitively, these three properties can be understood as 
- __Completeness__: The protocol "completes" (ends) correctly
- __Special Soundness__: If the verifier accepts it must be the case that the prover knows the witness (with large enough probability)
- __Special honest verifier zero knowledge (SHVZK)__: A transcript does not leak the witness (since the transcript could be efficiently simulated)

### Schnorr $\Sigma$-Protocol
In the Schnorr $\Sigma$-Protocol the verifier wants to prove that they have knowledge to a private key $`h = g^w`$ where $G = \langle g \rangle$ is a group of large prime order $q$. For the prover to show this, the following steps are taken: 
- First, the prover samples some randomness $`r \leftarrow \$Z_q`$ and computes the value $`a = g^r \in G`$. 
- Secondly, the Verifier samples some other value $`e \leftarrow \$Z_q`$ that they pass back to the Prover
- The prover calculates $`z = r + e \cdot w \in Z_q`$ and passes it to the verifier
- The verifier approves if $`g^z = a \cdot h^e`$

It is worth pointing out the properties we described in the previous section. If we have $(a,e,z)$, $(a,e',z')$ we can deduce $w$ through 
```math
\begin{equation*}
    w = (z-z') \cdot (e - e')^{-1} \mod q
\end{equation*}
```
meaning we have _special soundness_. Additionally, if we know $e$, we have that 
```math
\begin{align*}
    &\{ (a,e,z) : r \leftarrow \$Z_q; \; a = g^r; \; z = r + ew \} \\
    & \{ (a,e,z) : z \leftarrow \$Z_q; \; a = g^z h^{-e} \}
\end{align*}
```
thus guaranteeing SHVZK.

### Proving knowledge of Pedersen Commitments
Let $G = \langle g \rangle$ be a cyclic group of (large) prime order $q$, and $h$ be a random element in $G \backslash g$. Additionally, let $q,g,h$ be public information. Then the Pedersen commitment function is defined as: 
```math
\begin{equation*}
    Commit(m,r) = g^m h^r = c   
\end{equation*}
```
To verify whether $c$ is a well-formed commitment ($`c = g^m h^r)`$, the following steps are taken:
- The Prover generates $`r_1,r_2 \leftarrow \$Z_q`$ and builds $`a = g^{r_1} h^{r_2}`$ and sends to the Verifier
- The Verifier generates $`e \leftarrow \$Z_q^*`$ and returns to the Prover
- The prover calculates $`z_1 = r_1 + e \cdot m \in Z_q`$ and $`z_2 = r_2 + e \cdot r \in Z_q`$ and sends $`z = (z_1, z_2)`$ back to the verifier.
- Lastly, the verifier checks whether $`g^{z_1} h^{z_2} = c^e \cdot a`$.

### Schnorr $\Sigma$-Protocol (non-interactive)
One might also consider how to create a proof without having the verifier generate a challenge $e$. This can be accomplished by incorporating hashing, consider the following steps:
- The Prover generates some randomness $`r \leftarrow \$ Z_q`$ and creates the message $`a = g^r \in G`$
- We then use some hash function to generate the challenge $`e = H(g,x,a)`$
- Finally, we calculate $`z = r + e \cdot w \in Z_q`$


