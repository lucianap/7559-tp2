use std::sync::mpsc::{Sender, Receiver};
use std::string::String;
use std::vec::Vec;
use crate::logger::Logger;
use std::fmt::format;

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
            
            let mut txt = format!("Soy el hilo {} y recibi de {} informe {} pepitas", self.id, result.id_minero_sender, result.pepitas);
            logger.debug(&txt);
        } 
        return mensajes;
    }
}
