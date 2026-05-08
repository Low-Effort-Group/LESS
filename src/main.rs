mod startup;
use crate::startup::startup;
use rand;

mod types;

fn main() {
    startup();
    let id: usize = rand::random_range(0..2);
    crate::types::call_id(id);
    ()
}
