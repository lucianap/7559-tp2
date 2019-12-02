use std::string::String;

use crate::mapa;
use crate::minero_net::Mensaje;
use crate::logger::Logger;

pub struct Minero {
    pub nombre: String,
    pub id: i32,
    pub pepitas_obtenidas: i32,
    pub pepitas_acumuladas: i32,
    pub activo: bool
}

impl Minero {

    pub fn new(nombre: String, id: i32) -> Minero {
        Minero {
            nombre,
            id,
            pepitas_acumuladas: 0,
            pepitas_obtenidas: 0,
            activo: true
        }
    }

    pub fn explorar_porcion(&mut self, porcion: &mapa::Porcion, logger: &Logger) {
        self.pepitas_obtenidas = porcion.extraer(&logger);
        self.pepitas_acumuladas = self.pepitas_acumuladas + self.pepitas_obtenidas;
        let mut txt = format!("Minero {} extrae {} pepitas. Tiene acumuladas: {}", self.id,
                              self.pepitas_obtenidas,
                              self.pepitas_acumuladas);
        logger.debug(&txt)
    }

    pub fn get_pepitas_acumuladas(&self) -> &i32 {
        return &self.pepitas_acumuladas;
    }

    pub fn get_id(&self) -> &i32 {
        return &self.id;
    }

    pub fn agregar_pepitas(&mut self, cantidad:i32) {
        self.pepitas_acumuladas = self.pepitas_acumuladas + cantidad;
    }

    pub fn tengo_entregar_pepitas( &self , mensajes: &Vec<Mensaje>) -> bool{
        let mut minimo_obtenido = self.pepitas_obtenidas;
        let mut id_minero_minimo = self.id;
        for mensaje in mensajes {
            if mensaje.id_minero_sender != self.id && mensaje.pepitas <= minimo_obtenido {
                minimo_obtenido = mensaje.pepitas;
                id_minero_minimo = mensaje.id_minero_sender;
            }
        }
        return id_minero_minimo == self.id;
    }

    pub fn tengo_recibir_pepitas(&self , mensajes: &Vec<Mensaje>) -> bool {
        let mut maximo_obtenido = self.pepitas_obtenidas;
        let mut id_minero_maximo = self.id;
        for mensaje in mensajes {
            if mensaje.pepitas > maximo_obtenido {
                maximo_obtenido = mensaje.pepitas;
                id_minero_maximo = mensaje.id_minero_sender;
            } else if mensaje.pepitas == maximo_obtenido && 
                    mensaje.id_minero_sender > id_minero_maximo {
                id_minero_maximo = mensaje.id_minero_sender;
            }
        }

        return self.id == id_minero_maximo;
    }

}


#[cfg(test)]
mod test {

    use super::Minero;
    use crate::minero_net::Mensaje;
    use crate::minero_net::TipoMensaje;

    #[test]
    fn cuando_agregamos_pepitas_a_un_minero_se_debe_incrementar_lo_acumulado() {

        //given: Dado un minero con 5 pepitas acumuladas
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_acumuladas = 5;

        //when: Al agregar 10 pepitas
        minero.agregar_pepitas(10);

        //then: El minero debe acumular 15 pepitas
        let acumulado_esperado:i32 = 15;
        assert_eq!(acumulado_esperado, *minero.get_pepitas_acumuladas());

    }

    //a) Si el minero saco el minimo y es el unico, entonces entrega pepitas acumuladas

    #[test]
    fn si_un_minero_recolecta_el_minimo_y_es_el_unico_debe_entregar_pepitas() {

        //given: Un minero con 10 pepitas recolectadas y mensajes
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 5;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 10 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 20 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let entregar = minero.tengo_entregar_pepitas(&mensajes);
        
        //then: El minero 1, tien que entregar pepitas 
        
        assert!(entregar, "El minero no esta entregando pepitas");
    }

    #[test]
    fn si_un_minero_recolecta_el_minimo_y_es_no_es_unico_no_debe_entregar_pepitas() {

        //given: Un minero con 10 pepitas recolectadas y mensajes
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 5;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let entregar = minero.tengo_entregar_pepitas(&mensajes);
        
        //then: El minero 1, no tiene que entregar pepitas 
        
        assert!(!entregar, "El minero esta entregando pepitas y no deberia porque no es el unico con minimo");
    
    }

    #[test]
    fn si_otro_minero_recolecta_el_minimo_entonces_no_debo_entregar_pepitas() {

        //given: Un minero con 7 pepitas recolectadas y el minero 2 recolecta 5
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 7;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let entregar = minero.tengo_entregar_pepitas(&mensajes);
        
        //then: El minero 1, no tiene que entregar pepitas 
        
        assert!(!entregar, "El minero esta entregando pepitas y no deberia porque no tiene el minimo");
    
    }

    //b) Si soy el minero con mayor pepitas obtenidas, y se cumple a), espero transferencia de pepipas
    //c) Si hay dos o mas mineros con mayor pepitas obtenidas, el de mayor id espera transferencia de pepipas
    #[test]
    fn si_un_minero_recolecta_el_maximo_entonces_debo_recibir_pepitas() {

        //given: Un minero con 30 pepitas recolectadas
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 30;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let recibir = minero.tengo_recibir_pepitas(&mensajes);
        
        //then
        assert!(recibir, "El minero debe recibir pepitas porque tiene el maximo recolectado o el de mayor id")

    }

    #[test]
    fn si_hay_varios_mineros_con_el_maximo_entonces_debe_recibir_pepitas_el_de_mayor_id() {

        //given: Un minero con 30 pepitas recolectadas y pero no soy el mayor
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 30;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mensaje3 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 4, activo: true, pepitas: 30 };
        let mensaje4 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 30 };

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);
        mensajes.push(mensaje3);
        mensajes.push(mensaje4);


        //when: Evaluo si tengo que recibir pepitas
        let recibir = minero.tengo_recibir_pepitas(&mensajes);
        //let id_minero_maximo = minero.quien_tiene_maximo(&mensajes);

        //then: No tengo que recibir pepitas porque recolecte mas pero no tengo el menor id
        assert!(!recibir, "El minero no debe recibir pepitas porque tiene el maximo id")

    }

}
