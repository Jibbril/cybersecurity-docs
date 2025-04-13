mod mod_group;
mod utils;
mod participant;
mod server;

use mod_group::ModGroup;
use participant::Participant;
use server::Server;

const P: u64 = 233;
const G: u64 = 3;

fn main() {
    let group = ModGroup::new(P, G);

    let mut alice = Participant::new("Alice", &group);
    let mut bob = Participant::new("Bob", &group);

    let mut server = Server::new();
    server.register(&mut alice);
    server.register(&mut bob);

    let bob_keys = server.request_contact_details_for(&bob.name.clone());
    let contact_payload = alice.receive_keys(bob_keys);

    bob.receive_initial_message(contact_payload);
    
}
