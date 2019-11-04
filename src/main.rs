use std::io;
use rand::Rng;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

mod minero;

fn main() {

    let mut threadHandlers = vec![];

    for _number in 0..4 {

        let threadHandle = thread::spawn(|| {

            let n= minero::ejecutar();

            println!("rnd num {} ", n);

        });

        threadHandlers.push(threadHandle);

    }

    for threadHandler in threadHandlers {
        threadHandler.join().expect("failed to join thread");;
    }

}
