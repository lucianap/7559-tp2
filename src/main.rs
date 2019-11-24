use std::io;
use std::string::String;
use std::sync::{Arc, Barrier, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

mod minero;
mod mapa;

fn main() {

    //Estos tres valores deben ser pasados por parámetro.
    let CANTIDAD_MINEROS = 7;
    let CANTIDAD_REGIONES = 10;
    let MAX_PEPITAS_POR_REGION = 700;

    //Barrera que impide que los mineros sigan explorando la siguiente porción
    //y esperen a que todos terminen con la porción actual.
    let barrera_porcion = Arc::new(Barrier::new(CANTIDAD_MINEROS as usize));

    //Creación del mapa: lo pongo dentro de un ARC para poder compartirlo entre mineros.
    //AKA. cada minero obtiene una copia del mapa.
    let mapa: Arc<mapa::Mapa> = Arc::new(
        mapa::Mapa::crear(
        CANTIDAD_REGIONES,
        MAX_PEPITAS_POR_REGION));

    for (i, p) in mapa.porciones.iter().enumerate() {
        println!("Porción {} posee {} pepitas.", i, *p.pepitas.lock().unwrap());
    }

    let mut thread_handlers = vec![];

    //Canal de comunicación entre los mineros (senders) y el thread principal (reciever).
    let (tx, rx) = mpsc::channel();

    //Abro un thread por cada minero.
    for number in 0..CANTIDAD_MINEROS {

        //Clono el canal para poder ceder el ownership del lado transmisor.
        let thread_transmitter = mpsc::Sender::clone(&tx);

        //Clono el mapa -> uno para cada minero.
        let mi_mapa: Arc<mapa::Mapa> = Arc::clone(&mapa);

        let c = barrera_porcion.clone();

        //Lanzo los mineros
        let thread_handle = thread::spawn(move || {

            //Creación del minero. Mutable porque tiene atributos que cambian en cada exploración.
            let mut minero = minero::Minero::new("nombre".to_string(), number);

            //Comienzan las exploraciones.
            for n in 0..mi_mapa.total_porciones() {

                //Extraigo una porción del mapa.
                let mi_porcion = mi_mapa.obtener_porcion(n);

                //Minero extrae todas las pepitas del mapa.
                minero.explorar_porcion(&mi_porcion);

                //Envio por el canal qué minero soy y cuántas pepitas tengo acumuladas.
                let val = format!("Pepitas: {}", minero.get_pepitas_acumuladas());
                let min_id = minero.get_id().clone();

                let mensaje = (min_id, val);

                //envío valor al canal
                thread_transmitter.send(mensaje).unwrap();

                //Espero a que todos terminen.
                c.wait();

            }

        });

        thread_handlers.push(thread_handle);

    }

    //Recibo todos los mensajes que mandaron al canal
    for received in rx {
        let id = received.0;
        let message = received.1;
        println!("Mensaje: \"{}\" ; Del minero número: {} ", message, id);
    }

    for thread_handler in thread_handlers {
        thread_handler.join().expect("failed to join thread");
    }


}
