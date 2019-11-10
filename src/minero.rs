use rand::Rng;
use std::string::String;
use std::thread;

use crate::mapa;

pub struct Minero {
    pub nombre: String,
    pub id: i32,
    pub pepitas_obtenidas: i32,
    pub pepitas_acumuladas: i32
}

impl Minero {

    pub fn new(nombre: String, id: i32) -> Minero {
        Minero {
            nombre,
            id,
            pepitas_acumuladas: 0,
            pepitas_obtenidas: 0
        }
    }

    pub fn explorar_porcion(&mut self, porcion: &mapa::Porcion) {
        self.pepitas_obtenidas = porcion.extraer();
        self.pepitas_acumuladas = self.pepitas_acumuladas + self.pepitas_obtenidas;
        println!("Minero {} extrae pepitas. Cantidad: {}", self.id, self.pepitas_acumuladas);
    }

}

pub fn random_num() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

pub fn ejecutar() -> i32 {
    return random_num();
}










