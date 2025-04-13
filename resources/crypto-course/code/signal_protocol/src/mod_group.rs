use std::collections::HashSet;
use rand::seq::IteratorRandom; // Use the `rand` crate for random sampling

pub struct ModGroup {
    pub modulo: u64,
    pub generator: u64,
    pub elements: HashSet<u64>,
}

#[allow(dead_code)] 
impl ModGroup {
    pub fn new(p: u64, generator: u64) -> Self {
        let mut elements = HashSet::new();

        let mut current = 1;
        for _ in 0..(p - 1) {
            elements.insert(current);
            current = (current * generator) % p;
        }

        Self {
            modulo: p,
            generator,
            elements,
        }
    }

    pub fn contains(&self, element: u64) -> bool {
        self.elements.contains(&element)
    }

    pub fn elements(&self) -> HashSet<u64> {
        self.elements.clone()
    }

    pub fn elements_vec(&self) -> Vec<u64> {
        self.elements.iter().map(|&n| n).collect()
    }

    pub fn sample_n(&self, n: usize) -> Vec<u64> {
        if n > self.elements.len() {
            panic!("Requested more samples than size of group.");
        }

        let mut rng = rand::thread_rng();
        let sampled: Vec<u64> = self.elements.iter().cloned().choose_multiple(&mut rng, n);
        sampled
    }
}
