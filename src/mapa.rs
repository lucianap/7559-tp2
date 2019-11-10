use rand::Rng;


pub struct Mapa {
    pub num_porciones: usize,
    pub porciones: Vec<Porcion>
}

pub struct Porcion {
    pub pepitas: i32
}

impl Porcion {

    pub fn extraer(&self) -> i32{
        let cantidad_extraida = rand::thread_rng().gen_range(1, self.pepitas);
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
            let pepitas_en_porcion = rand::thread_rng().gen_range(1, *max_pepitas_por_porcion);
            porciones.push(
                Porcion{
                    pepitas: pepitas_en_porcion
                }
            );
        }

        return porciones;
    }

}



