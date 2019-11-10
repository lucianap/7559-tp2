use rand::Rng;
use std::sync::Mutex;

pub struct Mapa {
    pub num_porciones: usize,
    pub porciones: Vec<Porcion>
}

//Usa un mutex para hacerlo thread safe.
//Ya que varios mineros van a extraer en paralelo.
pub struct Porcion {
    pub pepitas: Mutex<i32>
}

impl Porcion {

    //Extrae pepitas y recalcula la cantidad de pepitas en la región.
    pub fn extraer(&self) -> i32 {
        let mut mut_pepitas = self.pepitas.lock().expect("No pudo obtenerse el mutex.");
        let cantidad_extraida = rand::thread_rng().gen_range(1, *mut_pepitas);
        let cantidad_nueva = (*mut_pepitas) - cantidad_extraida;
        *mut_pepitas = cantidad_nueva;
        return cantidad_extraida;
    }

}

impl Mapa {

    pub fn crear(num_porciones: usize, max_pepitas_por_porcion: i32) -> Mapa {
        Mapa {
            num_porciones,
            porciones: Mapa::crear_divisiones(&num_porciones, &max_pepitas_por_porcion)
        }
    }

    fn crear_divisiones(num_porciones: &usize, max_pepitas_por_porcion: &i32) -> Vec<Porcion> {

        let mut porciones = Vec::with_capacity(*num_porciones);

        for _porcion_n in 1..*num_porciones{
            let pepitas_en_porcion = rand::thread_rng().gen_range(0, *max_pepitas_por_porcion);
            porciones.push(
                Porcion{
                    pepitas: Mutex::new(pepitas_en_porcion)
                }
            );
        }

        return porciones;
    }

    pub fn extraer_porcion(&mut self) -> Porcion {
        let index = rand::thread_rng().gen_range(0, self.num_porciones-1);
        return self.porciones.remove(index);
    }

}



