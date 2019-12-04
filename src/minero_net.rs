use std::sync::mpsc::{Sender, Receiver};
use std::vec::Vec;
use crate::logger::Logger;

pub struct MineroNet {
    id:usize,
    senders: Vec<Sender<Mensaje>>,
    receiver: Receiver<Mensaje>,
}
#[derive(Copy, Clone)]
pub enum TipoMensaje {
    Intercambio,
    Informacion
}

#[derive(Copy, Clone)]
pub struct Mensaje {
    pub tipo_operacion: TipoMensaje,
    pub id_minero_sender: i32,
    pub activo: bool,
    pub pepitas: i32
}


impl MineroNet {

    pub fn new(id:usize, senders: Vec<Sender<Mensaje>>, receiver: Receiver<Mensaje>) -> MineroNet {
        MineroNet {
            id,
            senders,
            receiver,
        }
    }

    pub fn notificar_todos(&mut self, valor: Mensaje) {
        let cant_hilos = self.senders.len();    
        for j in 0..cant_hilos {
            if j != self.id {
                self.senders[j].send(valor).unwrap();

            }
        }
    }

    pub fn escuchar_todos(&mut self, logger: &Logger) -> Vec<Mensaje> {
        let cant_hilos = self.senders.len();    
        let mut mensajes:Vec<Mensaje> = Vec::new();
        for _ in 0..cant_hilos-1 {
            let result = self.receiver.recv().unwrap();
            mensajes.push(result);
            
//            let txt = format!("Soy el minero {} y recibi de {} que informe {} pepitas", self.id, result.id_minero_sender, result.pepitas);
//            logger.debug(&txt);
        } 
        return mensajes;
    }

    pub fn recibir_pepitas(&mut self, logger: &Logger) -> Mensaje {

        let mensaje = self.receiver.recv()
                    .expect("No se pudo recibir el mensaje");
        let txt = format!("RECIBIR pepitas {} DE minero {}.", mensaje.pepitas, mensaje.id_minero_sender);
        logger.info(&txt);
        return mensaje;
    }

    pub fn enviar_a(&mut self, id_minero_destino: usize, mensaje : Mensaje,  logger: &Logger) {
        let txt = format!("ENVIAR pepitas {} A minero {}.", mensaje.pepitas, id_minero_destino);
        logger.info(&txt);
        self.senders[id_minero_destino].send(mensaje)
            .expect("No se pudo entregar el mensaje a minero detino");
    }
}
