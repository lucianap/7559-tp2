use std::io;
use rand::Rng;
use std::string::String;

fn main() {

    let n= random_num();

    println!("Número random {}!", n);

}

fn random_num() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}