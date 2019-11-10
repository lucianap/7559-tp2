use rand::Rng;
use std::string::String;
use std::thread;

use crate::mapa;

pub struct Minero {
    pub nombre: String,
    pub id: i32,
    pub pepitas: i32
}

impl Minero {

    pub fn new(nombre: String, id: i32) -> Minero {
        Minero {
            nombre,
            id,
            pepitas: 0
        }
    }

    pub fn explorar_porcion(&mut self, porcion: mapa::Porcion) {
        self.pepitas = porcion.extraer();
    }

}

pub fn random_num() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

pub fn ejecutar() -> i32 {
    return random_num();
}










