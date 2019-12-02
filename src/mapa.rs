use rand::Rng;

use std::sync::{Arc, Mutex};
use crate::logger::Logger;

pub struct Mapa {
    pub num_porciones: usize,
    pub porciones: Vec<Arc<Porcion>>
}

//Usa un mutex para hacerlo thread safe.
//Ya que varios mineros van a extraer en paralelo.
pub struct Porcion {
    pub pepitas: Mutex<i32>
}

impl Porcion {

    //Extrae pepitas y recalcula la cantidad de pepitas en la región.
    pub fn extraer(&self,logger: &Logger) -> i32 {
        let mut mtx_pepitas = self.pepitas.lock().expect("No pudo obtenerse el mutex.");
        if *mtx_pepitas > 0 {
            let cantidad_extraida = rand::thread_rng().gen_range(0, *mtx_pepitas);
            let cantidad_nueva = (*mtx_pepitas) - cantidad_extraida;
            let mut txt = format!("Extracción de pepitas. Se extraen: {} pepitas. Quedan {}.", cantidad_extraida, cantidad_nueva);
            logger.debug(&txt);
            *mtx_pepitas = cantidad_nueva;
            return cantidad_extraida;
        } else {
            return 0;
        }
    }

}

impl Mapa {
    /**
        Crea un mapa con <num_porciones>, cada porción tendrá <n> pepitas.
    */
    pub fn crear(num_porciones: usize, max_pepitas_por_porcion: i32) -> Mapa {
        Mapa {
            num_porciones,
            porciones: Mapa::crear_divisiones(&num_porciones, &max_pepitas_por_porcion)
        }
    }

    /**
        Función privada.
        Divide el mapa en <num_porciones>, cada una de esas pociones tendrá en n pepitas
        siendo n un número random entre 1 y <max_pepitas_por_porcion>
    */
    fn crear_divisiones(num_porciones: &usize, max_pepitas_por_porcion: &i32) -> Vec<Arc<Porcion>> {
        let mut porciones = Vec::with_capacity(*num_porciones);
        for _porcion_n in 1..*num_porciones{
            let pepitas_en_porcion = rand::thread_rng().gen_range(0, *max_pepitas_por_porcion);
            porciones.push(Arc::new(
                Porcion{ pepitas: Mutex::new(pepitas_en_porcion) }
            ));
        }
        return porciones;
    }

    /**
        Devuelve la porción <num_porción> del mapa.
    */
    pub fn obtener_porcion(&self, num_porcion: usize) -> Arc<Porcion> {
        return Arc::clone(&self.porciones[num_porcion]);
    }

    /**
        Devuelve el total de porciones que hay en el mapa.
    */
    pub fn total_porciones(&self) -> usize {
        return self.porciones.len();
    }

}



