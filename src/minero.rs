use rand::Rng;
use std::string::String;
use std::thread;

pub struct Minero {
    pub nombre: String,
    pub id: i32
}

pub fn random_num() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

pub fn ejecutar() -> i32 {
    return random_num();
}




