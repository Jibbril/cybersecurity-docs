use crate::participant::Participant;

pub struct Server {
    participants: Vec<(String, u64, u64, Vec<u64>)>
}

impl Server {
    pub fn new() -> Self {
        Self {
            participants: vec![]
        }
    }

    pub fn register(&mut self, new_participant: &Participant) {
        let name = new_participant.name.clone();
        let pk = new_participant.pk;
        let mtpk = new_participant.pk;
        let ephpks = new_participant.ephpks.clone();

        self.participants.push((name, pk, mtpk, ephpks));
    }

    pub fn request_contact_details_for(&mut self, to: &str) -> (u64, u64, (usize, u64)) {
        let mut recipient: Vec<_> = self.participants.iter_mut()
            .filter(|p| p.0 == to)
            .collect();

        if recipient.len() != 1 {
            panic!("Unable to find recipient when initiating contact.");
        }

        let (_, pk, mtpk, ephpks): &mut (String, u64, u64, Vec<u64>) = recipient[0];

        let ephpk = ephpks.pop().expect("Expected ephpk when requesting contact details.");

        (*pk,*mtpk,(ephpks.len(),ephpk))
    }
}
