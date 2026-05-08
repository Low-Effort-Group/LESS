mod startup;
use crate::startup::startup;

mod types;
use crate::types::CONTENT_BALL;

fn main() {
    startup();
    println!("{:?}", CONTENT_BALL);
    CONTENT_BALL.invoke(Vec::from([1, 2]));
    ()
}
