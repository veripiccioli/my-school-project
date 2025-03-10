// This code will randomly generate a string of length 10
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let random_string: String = std::iter::repeat(())
        .take(10)
        .map(|()| rng.sample(Alphanumeric))
        .collect();
    
    println!("{}", random_string);
}
