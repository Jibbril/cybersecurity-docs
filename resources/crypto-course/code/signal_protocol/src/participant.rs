use crate::{mod_group::ModGroup, utils::mod_exp};

#[allow(dead_code)] 
pub struct Participant<'a> {
    pub name: String,
    pub pk: u64,
    pub mtpk:u64,
    pub ephpks: Vec<u64>,
    sk: u64,
    mtsk: u64,
    ephsks: Vec<u64>,
    ms: Option<u64>,
    group: &'a ModGroup
}

impl <'a> Participant<'a> {
    pub fn new(name: &str, group: &'a ModGroup) -> Self{
        let mut secret_keys = group.sample_n(10);
        let mut public_keys: Vec<u64> = secret_keys.iter()
            .map(|k| mod_exp(group.generator, *k, group.modulo))
            .collect();
        
        Self {
            name: name.to_string(),
            pk: public_keys.pop().unwrap(),
            sk: secret_keys.pop().unwrap(),
            mtpk: public_keys.pop().unwrap(),
            mtsk: secret_keys.pop().unwrap(),
            ephpks: public_keys,
            ephsks: secret_keys,
            group,
            ms: None,
        }
    }

    pub fn receive_keys(&mut self, keys: (u64, u64, (usize, u64))) -> (usize, (u64, u64)) {
        let (pk, mtpk, (i,ephpk)) = keys;

        let ephsk = self.ephsks.pop().expect("Expected ephsk when receiving keys.");
        let self_ephpk = self.ephpks.pop().expect("Expected ephpk when receiving keys.");

        let mut ms = 1;
        ms *= mod_exp(mtpk, self.sk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(pk, ephsk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(mtpk, ephsk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(ephpk, ephsk, self.group.modulo) % self.group.modulo;

        self.ms = Some(ms);

        println!("{} generated ms: {}", self.name, ms);

        let ad = (self.pk, self_ephpk);

        (i,ad)
    }

    pub fn receive_initial_message(&mut self, (i, ad): (usize,(u64,u64))) {
        let (pk, ephpk) = ad;

        let ephsk = self.ephsks.remove(i);
        let _ = self.ephpks.remove(i);

        let mut ms = 1;
        ms *= mod_exp(pk, self.mtsk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(ephpk, self.sk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(ephpk, self.mtsk, self.group.modulo) % self.group.modulo;
        ms *= mod_exp(ephpk, ephsk, self.group.modulo) % self.group.modulo;

        self.ms = Some(ms);

        println!("{} generated ms: {}", self.name, ms);
    }
}
