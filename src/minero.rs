use std::io;
use rand::Rng;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

pub fn random_num() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

pub fn ejecutar() -> i32 {
    return random_num();
}




