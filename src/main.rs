use std::io;
use rand::Rng;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::sync::mpsc;

mod minero;


fn main() {

    let CANTIDAD_MINEROS = 7;

    let mut thread_handlers = vec![];

    let (tx, rx) = mpsc::channel();

    //abro un thread por cada minero.
    for number in 0..CANTIDAD_MINEROS {

        //Clono el canal para poder ceder el ownership.
        let thread_transmitter = mpsc::Sender::clone(&tx);

        let thread_handle = thread::spawn(move || {

            let min = minero::Minero{nombre:String::from("minero"), id: number};

            //minero elige un número random.
            let n = minero::ejecutar();

            let message = "Random : ";
            let val = format!("{}{}", message, n);

            let mensaje = (min, val);

            //envío valor al canal
            thread_transmitter.send(mensaje).unwrap();

        });

        thread_handlers.push(thread_handle);

    }

    for thread_handler in thread_handlers {
        thread_handler.join().expect("failed to join thread");
    }

    //Recibo todos los mensajes que mandaron al canal
    for received in rx {
        let min = received.0;
        let message = received.1;
        println!("Got: \"{}\" ; From: {} - {} ", message, min.nombre, min.id);
    }

}
