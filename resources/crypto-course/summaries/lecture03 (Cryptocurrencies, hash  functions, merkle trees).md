# Lecture 2

### Cryptocurrencies
The current financial system is dependent on trusted third-party actors (banks) to manage transactions, accounts, etc. With cryptocurrencies, these are replaced by trust established by the network itself. 

The three important properties for a blockchain is 
- It is distributed
- It is immutable
- It has some consensus mechanism to resolve forks

### The Bitcoin hash puzzle (proof of work)
The bitcoin hash puzzle consists of encrypting a block with the SHA256 hash algorithm while replacing the nonce inside the block until the hash digest starts with a given number of zeroes. The number of zeroes required for a passing block depends on the total computation power that is available in the system at any one time. If there is a lot of compute, the number of zeroes rises, if there is less compute the number of zeroes falls. This is how the Bitcoin network ensures consistency of block production. 

### Consensus mechanisms in Bitcoin
The main rule for consensus in Bitcoin is that the "longest chain wins". So in the case of a fork, the network will continue building on both forks until one is provably longer than the other. Because of this, verification of a transaction can sometimes take several hours (5-10 blocks). 

### Sybil attacks
A sybil attack is an attack on a concensus mechanism where the same person creates multiple accounts to vote illegitimately many times. Bitcoin protects itself from these kinds of attacks through the proof of work concept. To be able to enact 51% attacks you would need to own 51% of the compute available in the network. 

### Bitcoin and the environment
Due to the massive amount of electricity used globally to compute in and sustain the Bitcoin network it can easily be argued that Bitcoin is bad for the environment. Some have argued that the proof-of-work system is a major problem for this exact reason, and should be replaced by the less computationally demanding proof-of-stake paradigm. 

### Proof-of-stake
In proof of stake, the key concept is that of a lottery. Everyone who stakes currency in the system joins the lottery, with chance to win proportional to their stake. If you win the lottery, you get to validate a block and reap the block rewards. In this way, the actors that have staked the most will have the largest chance to validate blocks, but they also have the most to lose in case they approve fraudulent blocks due to slashing (taking of the staked currency). Thus they are heavily incentivized to keep the network runnning in a correct way. 

### Anonymity in Bitcoin
Bitcoin is simultaneously completely anonymous and very transparent. This is because creating a bitcoin wallet is completely anonymous, but if that wallet is somehow linked to a person then the entire transaction history is publicly available. There are other cryptocurrencies like Zcash that are focused on true privacy that are based on Zero-Knowledge Proofs.

### Hash functions
Hash functions should be easy to compute, but hard to invert. A good allegory is to that of a mixer; it's easy to but ingredients in and press start to mix them up, but it is very hard to take the finished mix and try to piece out the ingredients. 

### One-way functions
A function $`f:\{0,1\}^n \rightarrow \{0,1\}^d`$ is one-way if:
- $f(x)$ can be efficiently computed for all $`x \in \{0,1\}^n`$
- There exists a negligible function $negl_A(x)$ for all probabalistic polynomial time algorithms $A$ such that for sufficiently large values of $n \in N$ we have that $`Pr[f(x) = f(x') | x \leftarrow \$\{0,1\}^n, x' \leftarrow A(f(x))] \leq negl_A(n)`$.

### Cryptographic hash functions
A cryptographic hash function $H$ is a particular kind of one-way function that in addition to the properties of the one-way-function also have that 

- $|H(x)| < |x|$ where the length of $x$ is $n$ and the length of $H(x)$ is $d$. 
- $H$ is collision resistant, meaning that $`Pr[H(x) = H(x') | x,x' \leftarrow A(H), x \neq x'] \leq negl(n)`$

### Estimating collisions for hash functions (birthday attacks)
There are 365 days in a year. Given a group of $n$ people, what are the odds that at least two people have the same birthday? To compute this we take the following steps:

- There is a $`1 - \frac{1}{365} = \frac{364}{365}`$ probability that a given pair of birthdays are different.
- For n people there are $`\binom{n}{2} = \frac{n (n - 1)}{2}`$ pairs
- The probability of not having a collision then becomes $`P = \left( \frac{364}{365} \right)^{\frac{n (n-1)}{2}}`$.
- To get the chance of collision we can now simply subtract $1 - P$

### Merkle trees
A Merkle tree is a data structure that simplifies the storage of data by continuously hashing the contents together and only saving the top hash (the merkle root). It is similar to a Blockchain in the sense that it immutably encrypts data in a way where if some change was made it would instantly be discovered due to the further hashes not computing correctly. One example of an implementation of this is git. 
