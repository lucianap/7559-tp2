use rand::Rng;
use std::io;
use std::string::String;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

mod minero;
mod mapa;

fn main() {

    let CANTIDAD_MINEROS = 7;
    let CANTIDAD_REGIONES = 10;

    let mapa: mapa::Mapa = mapa::Mapa::crear(CANTIDAD_REGIONES, 89);

    for (i, p) in mapa.porciones.iter().enumerate() {
        println!("Porción {} posee {} pepitas.", i, p.pepitas);
    }

    let mut thread_handlers = vec![];

    let (tx, rx) = mpsc::channel();

    //abro un thread por cada minero.
    for number in 0..CANTIDAD_MINEROS {

        //Clono el canal para poder ceder el ownership.
        let thread_transmitter = mpsc::Sender::clone(&tx);

        let thread_handle = thread::spawn(move || {

            let min = minero::Minero::new("nomber".to_string(), number);

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
