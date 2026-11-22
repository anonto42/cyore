// mod data;
mod random;

use random::Rng;

fn main() {
    
    let mut rng = Rng::new(42);

    for _ in 0..5 {
        println!{"{:?}", rng.range(-1.0, 1.0)};
    }
   
}
