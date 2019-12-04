use std::string::String;

use crate::mapa;
use crate::minero_net::Mensaje;
use crate::logger::Logger;
use std::collections::BTreeMap;
use crate::minero_net::TipoMensaje;


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
        let txt = format!("Minero {} extrae {} pepitas. Tiene acumuladas: {}", self.id,
                              self.pepitas_obtenidas,
                              self.pepitas_acumuladas);

        logger.debug(&txt);
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

        if !self.activo { return false; }

        let mut minimo_obtenido = self.pepitas_obtenidas;
        let mut id_minero_minimo = self.id;
        let mut cant_mensajes_activos = 0;
        for mensaje in mensajes {
            if !mensaje.activo { continue; }

            if mensaje.id_minero_sender != self.id && mensaje.pepitas <= minimo_obtenido {
                minimo_obtenido = mensaje.pepitas;
                id_minero_minimo = mensaje.id_minero_sender;
            }
            cant_mensajes_activos += 1;
        }
        return  cant_mensajes_activos != 0 && id_minero_minimo == self.id;
    }

    pub fn tengo_recibir_pepitas(&self , mensajes: &Vec<Mensaje>) -> bool {

        if !self.activo { return false; }

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

        return self.id == id_minero_maximo && !hay_multiples_minimos(&mensajes);
    }

    pub fn queda_un_minero( &self , mensajes: &Vec<Mensaje>, mi_logger: &Logger) -> bool{
        let mut mineros = 0;
        if self.activo {mineros += 1};

        mi_logger.debug(&format!("Minero {} imprime sus mensajes: ", self.id));

        for mensaje in mensajes {
            if mensaje.activo {mineros += 1};


            let tipo_m = match mensaje.tipo_operacion {
                TipoMensaje::Informacion => "info",
                TipoMensaje::Intercambio => "inter"

            };
            mi_logger.debug(&format!("Mensaje. Id minero:{}\tTipo mensaje:{}\tActivo:{}\tPepitas:{}",
                                     mensaje.id_minero_sender, tipo_m, mensaje.activo, mensaje.pepitas));
        }
        return mineros == 1;
    }
}

#[warn(unused_assignments)]
pub fn obtener_id_minero_destino( mensajes: &Vec<Mensaje>) -> i32 {
    let mut id_minero_maximo = 0; 

    if !mensajes.is_empty() {
        let mut maximo_obtenido = 0;
        maximo_obtenido = mensajes[0].pepitas;
        id_minero_maximo = mensajes[0].id_minero_sender;     
        
        for mensaje in mensajes {
            if mensaje.pepitas > maximo_obtenido {
                maximo_obtenido = mensaje.pepitas;
                id_minero_maximo = mensaje.id_minero_sender;
            } else if mensaje.pepitas == maximo_obtenido && 
                    mensaje.id_minero_sender > id_minero_maximo {
                id_minero_maximo = mensaje.id_minero_sender;
            }
        } 
    }

    return id_minero_maximo; 
}

pub fn hay_multiples_minimos(mensajes : &Vec<Mensaje>) -> bool{
    let mut map:BTreeMap<i32, i32> = BTreeMap::new();
    for mensaje in mensajes {
        if !mensaje.activo  { continue; }
        if !map.contains_key(&mensaje.pepitas) {
            map.insert(mensaje.pepitas, 1);
        } else {
            map.insert(mensaje.pepitas, map.get(&mensaje.pepitas)
                                    .and_then(|a| Some(a+1)).unwrap());
        }
    }
    if map.len() == 0 { return false }

    let (_, contador_minimos) = map.iter().next().unwrap();

    return *contador_minimos > 1;
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
    fn si_un_minero_recolecta_el_minimo_y_los_otros_estan_inactivos_no_debe_entregar_pepitas() {

        //given: Un minero con 2 pepitas recolectadas y mensajes
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 2;
        minero.activo = true;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: false, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: false, pepitas: 10 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros estan inactivos
        let entregar = minero.tengo_entregar_pepitas(&mensajes);
        
        //then: El minero 1, no tiene que entregar pepitas 
        
        assert!(!entregar, "El minero esta entregando pepitas y no deberia porque es el unico activo");
    
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
        assert!(recibir, "El minero debe recibir pepitas porque tiene el maximo recolectado o el de mayor id");
    
    }


    //b) Si soy el minero con mayor pepitas obtenidas, y se cumple a), espero transferencia de pepipas
    #[test]
    fn si_un_minero_recolecta_el_maximo_pero_hay_varios_minimos_entonces_no_debo_recibir_pepitas() {

        //given: Un minero con 30 pepitas recolectadas y otros dos mineros con igual minimo de recolecion de pepitas
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 30;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mensaje3 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 4, activo: true, pepitas: 5 };        

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);
        mensajes.push(mensaje3);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let recibir = minero.tengo_recibir_pepitas(&mensajes);

        //then
        assert!(!recibir, "El minero no debe recibir pepitas porque hay mineros con el mismo minimo de recoleccion");
    
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
        let id_minero_maximo = super::obtener_id_minero_destino(&mensajes);

        //then: No tengo que recibir pepitas porque recolecte mas pero no tengo el menor id
        assert!(!recibir, "El minero no debe recibir pepitas porque tiene el maximo id");
        assert_eq!(4, id_minero_maximo, "El id de minero no corresponde con el maximo id de los mensajes");
    
    }

    #[test]
    fn si_hay_multiples_mensajes_el_mismo_minimo_de_pepitas_recolectadas_debe_retornar_verdadero(){

        // given: Dado un listado de mensajes con dos mensajes con el mismo minimo de pepitas recolectas

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 5 };
        let mensaje3 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 4, activo: true, pepitas: 30 };
        let mensaje4 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 30 };

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);
        mensajes.push(mensaje3);
        mensajes.push(mensaje4);

        // when: cuando detectamamos dos minimos en dos mensajes

        let varios_minimos = super::hay_multiples_minimos(&mensajes);

        // given: Es verdad que hay varios minimos

        assert!(varios_minimos,"Tienen que encontrarse varios minimos en los mensajes");
    }
    #[test]
    fn si_hay_multiples_mensajes_y_un_unico_minimo_de_pepitas_recolectadas_debe_retornar_falso(){

        // given: Dado un listado de mensajes 

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };
        let mensaje3 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 4, activo: true, pepitas: 20 };
        let mensaje4 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 30 };

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);
        mensajes.push(mensaje3);
        mensajes.push(mensaje4);

        // when: cuando consultamos si hay multiples minimos

        let varios_minimos = super::hay_multiples_minimos(&mensajes);

        // given: Es falso que todos los mensajes estan inactivos

        assert!(!varios_minimos,"Tienen que omitirse todos los mensajes");
    }

    #[test]
    fn si_hay_multiples_mensajes_y_son_todo_inactivos_debe_retornar_falso(){

        // given: Dado un listado de mensajes 

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: false, pepitas: 10 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: false, pepitas: 10 };
        let mensaje3 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 4, activo: false, pepitas: 15 };
        let mensaje4 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: false, pepitas: 15 };

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);
        mensajes.push(mensaje3);
        mensajes.push(mensaje4);

        // when: cuando consultamos si hay multiples minimos

        let varios_minimos = super::hay_multiples_minimos(&mensajes);

        // given: Es falso que hay varios minimos

        assert!(!varios_minimos,"Tienen que encontrarse varios minimos en los mensajes");
    }

    // Si el minero esta inactivo, no debe entregar pepitas
    // Si el minero esta inactivo, no debe recibir pepitas

    #[test]
    fn si_un_minero_esta_inactivo_entonces_no_debo_recibir_pepitas() {

        //given: Un minero con 30 pepitas recolectadas y otros dos mineros con igual minimo de recolecion de pepitas
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 30;
        minero.activo = false;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 5 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 10 };

        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let recibir = minero.tengo_recibir_pepitas(&mensajes);

        //then
        assert!(!recibir, "El minero no debe recibir pepitas porque esta inactivo");
    
    }

    // cualquier mensaje de minero inactivo, se omite


    #[test]
    fn si_un_minero_esta_inactivo_entonces_no_debe_entregar_pepitas() {

        //given: Un minero con 10 pepitas recolectadas y mensajes
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 5;
        minero.activo = false;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 10 };        
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 20 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: Los mensajes indican que el resto de los mineros recolecto mas
        let entregar = minero.tengo_entregar_pepitas(&mensajes);
        
        //then: El minero 1, tien que entregar pepitas 
        
        assert!(!entregar, "El minero no tiene que entregar pepitas");
    }

    #[test]
    fn notificar_si_queda_un_solo_minero_para_salir() {

        //given: tengo mensajes de dos mineros  con unos olo activo
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 5;
        minero.activo = true;
        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: false, pepitas: 0 };
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: false, pepitas: 20 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: los mensajes dicen que solo uno esta activo
        let queda_uno = minero.queda_un_minero(&mensajes);

        //then: el minero notifica que queda un solo minero activo

        assert!(queda_uno, "Queda un solo minero");
    }

    #[test]
    fn notificar_si_queda_mas_de_un_solo_minero_para_salir() {

        //given: tengo mensajes de dos mineros  con ambos activos
        let mut minero:Minero = Minero::new("Pedro".to_string(), 1);
        minero.pepitas_obtenidas = 5;
        minero.activo = true;

        let mensaje1 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 2, activo: true, pepitas: 0 };
        let mensaje2 = Mensaje { tipo_operacion: TipoMensaje::Informacion, id_minero_sender: 3, activo: true, pepitas: 20 };
        let mut mensajes:Vec<Mensaje> = Vec::new();
        mensajes.push(mensaje1);
        mensajes.push(mensaje2);

        //when: los mensajes dicen que no solo uno esta activo
        let queda_uno = minero.queda_un_minero(&mensajes);

        //then: el minero notifica que no queda un solo minero activo

        assert!(!queda_uno, "Queda mas de un minero");
    }


    
}
