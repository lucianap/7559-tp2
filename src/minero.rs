use std::string::String;

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
        println!("Minero {} extrae {} pepitas. Tiene acumuladas: {}", self.id,
                 self.pepitas_obtenidas,
                 self.pepitas_acumuladas);
    }

    pub fn get_pepitas_acumuladas(&self) -> &i32 {
        return &self.pepitas_acumuladas;
    }

    pub fn get_id(&self) -> &i32 {
        return &self.id;
    }

}













