use std::sync::mpsc::{Sender, Receiver};

pub struct MineroNet {
    id:usize,
    senders: Vec<Sender<i32>>,
    receiver: Receiver<i32>,
}


impl MineroNet {

    pub fn new(id:usize, senders: Vec<Sender<i32>>, receiver: Receiver<i32>) -> MineroNet {
        MineroNet {
            id,
            senders,
            receiver,
        }
    }

    pub fn notificar_todos(&mut self, valor: i32) {
        let cant_hilos = self.senders.len();    
        for j in 0..cant_hilos {
            if j != self.id {
                self.senders[j].send(valor).unwrap();
            }
        }
    }

    pub fn escuchar_todos(&mut self) {
        let cant_hilos = self.senders.len();    
        for _ in 0..cant_hilos-1 {
            let result = self.receiver.recv().unwrap();
            println!("Soy el hilo {} y recibi {}", self.id, result);
        } 
    }
}
