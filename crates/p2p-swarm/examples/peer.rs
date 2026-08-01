//! Spins up two local peer handles and prints their identities.

use p2p_swarm::Swarm;

fn main() {
    let a = Swarm::with_local_id("peer-a");
    let b = Swarm::with_local_id("peer-b");
    println!("peer A: {:?}", a.local_id);
    println!("peer B: {:?}", b.local_id);
    println!("p2p-swarm stub ready — transport lands in a future phase");
}
