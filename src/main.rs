use std::io;
use std::string::String;
use std::sync::{Arc, Barrier, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::vec::Vec;
extern crate clap;
use clap::{Arg, App, SubCommand};
use crate::minero_net::Mensaje;
use crate::minero_net::TipoMensaje;
use crate::logger::Logger;

mod minero;
mod logger;
mod mapa;
mod minero_net;

fn main() {
    let matches = App::new("Proceso de minado")
        .version("1.0")
        .author("Nestor Hualpa, Ignacio Manes y Luciana")
        .about("Proceso de minado desarrollado para el trabajo practico 2 de Tecnicas de Programacion Concurrentes")
        .arg(Arg::with_name("PEPITAS")
            .short("p")
            .long("pepitas")
            .required(true)
            .help("Pepitas por region")
            .takes_value(true))
        .arg(Arg::with_name("REGIONES")
            .short("r")
            .long("regiones")
            .help("Cantidad de regiones")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("MINEROS")
            .short("m")
            .long("mineros")
            .help("cantidad de mineros")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("DEBUG")
            .short("d")
            .long("debug")
            .help("activa los logs de debug"))
        .get_matches();

    //Estos tres valores deben ser pasados por parámetro.
    let debug =  matches.is_present("DEBUG");
    let logger: Arc<logger::Logger> = Arc::new( logger::Logger::new(debug));
    let cantidad_mineros = matches.value_of("MINEROS").unwrap().parse().unwrap();
    let cantidad_regiones = matches.value_of("REGIONES").unwrap().parse().unwrap();
    let max_pepitas_por_region = matches.value_of("PEPITAS").unwrap().parse().unwrap();

    //Barrera que impide que los mineros sigan explorando la siguiente porción
    //y esperen a que todos terminen con la porción actual.
    let barrera_porcion = Arc::new(Barrier::new(cantidad_mineros));

    //Creación del mapa: lo pongo dentro de un ARC para poder compartirlo entre mineros.
    //AKA. cada minero obtiene una copia del mapa.
    let mapa: Arc<mapa::Mapa> = Arc::new(
        mapa::Mapa::crear(
            cantidad_regiones ,
            max_pepitas_por_region));

    for (i, p) in mapa.porciones.iter().enumerate() {
        logger.debug(&format!("Porción {} posee {} pepitas.", i, *p.pepitas.lock().unwrap()))
    }

    let mut thread_handlers = vec![];

    //Canal de comunicación entre los mineros (senders) y el thread principal (reciever).
    let (tx, rx) = mpsc::channel();

    // inicializacion de canales para la red de mineros
    let mut receivers = Vec::new();
    let mut senders = Vec::new();
    for _ in 0.. cantidad_mineros{
        let (tx, rx): (Sender<Mensaje>, Receiver<Mensaje>) = mpsc::channel();
        receivers.push(rx);
        senders.push(tx);
    }
    receivers.reverse();

    //Abro un thread por cada minero.
    for number in 0.. cantidad_mineros{

        //Clono el canal para poder ceder el ownership del lado transmisor.
        let thread_transmitter = mpsc::Sender::clone(&tx);

        //Clono el mapa -> uno para cada minero.
        let mi_mapa: Arc<mapa::Mapa> = Arc::clone(&mapa);

        //Clono el logger -> uno por minero
        let mi_logger: Arc<logger::Logger> = Arc::clone(&logger);

        let c = barrera_porcion.clone();

        // clono para el mover el ownership a los hilos
        let mut senders_copy = Vec::new();
        for elem in &senders {
            senders_copy.push(elem.clone());
        }
        // remuevo para pasar el ownership al hilo
        let rx = receivers.pop().unwrap();

        //Lanzo los mineros
        let thread_handle = thread::spawn(move || {

            //Creación del minero. Mutable porque tiene atributos que cambian en cada exploración.
            let mut minero = minero::Minero::new("nombre".to_string(), number as i32);

            //Creacion del la red minero para cada minero para comunicación
            let mut minero_hub = minero_net::MineroNet::new(number as usize, senders_copy, rx);

            //Comienzan las exploraciones.
            for n in 0..mi_mapa.total_porciones() {

                //Extraigo una porción del mapa.
                let mi_porcion = mi_mapa.obtener_porcion(n);

                if minero.activo {
                    //Minero extrae todas las pepitas del mapa.
                    minero.explorar_porcion(&mi_porcion,&mi_logger);
                }

                //Envio por el canal qué minero soy y cuántas pepitas tengo acumuladas.
                let val = format!("Pepitas: {}", minero.get_pepitas_acumuladas());
                let min_id = minero.get_id().clone();

                let mensaje = (min_id, val);

                //envío valor al canal
                thread_transmitter.send(mensaje).unwrap();

                //envio un valor al resto de los mineros y escucho una respuesta de todos
                let mensaje = Mensaje {
                    tipo_operacion: TipoMensaje::Informacion,
                    id_minero_sender: minero.id,
                    activo: minero.activo,
                    pepitas: minero.pepitas_obtenidas
                };

                minero_hub.notificar_todos(mensaje);

                let mensajes = minero_hub.escuchar_todos(&mi_logger);

                //Espero a que todos terminen.
                c.wait();
                
                if minero.tengo_recibir_pepitas(&mensajes) {
                    let txt = format!("Minero {} tiene que recibir pepitas", minero.id);
                    mi_logger.debug(&txt);
                    let mensaje:Mensaje = minero_hub.recibir_pepitas(&mi_logger);
                    minero.agregar_pepitas(mensaje.pepitas);

                } else if minero.tengo_entregar_pepitas(&mensajes) {
                    
                    //envio acumulado
                    let mensaje = Mensaje {
                        tipo_operacion: TipoMensaje::Intercambio,
                        id_minero_sender: minero.id,
                        activo: minero.activo,
                        pepitas: *minero.get_pepitas_acumuladas(),
                    };

                    let txt = format!("Minero {} tiene que entregar {} pepitas", minero.id, minero.pepitas_acumuladas);
                    mi_logger.debug(&txt);

                    let id_minero_desitino = minero::obtener_id_minero_destino(&mensajes);
                    minero_hub.enviar_a(id_minero_desitino as usize, mensaje, &mi_logger);
                    minero.activo = false;
                }
                
                c.wait();

            }

        });

        thread_handlers.push(thread_handle);

    }

    //Recibo todos los mensajes que mandaron al canal
    for received in rx {
        let id = received.0;
        let message = received.1;
        logger.debug(&format!("Mensaje: \"{}\" ; Del minero número: {} ", message, id));
    }

    for thread_handler in thread_handlers {
        thread_handler.join().expect("failed to join thread");
    }
}
