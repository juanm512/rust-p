// 9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
// pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
// Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
// se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
// dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
// desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
// diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
// Dado todo lo mencionado anteriormente implemente los métodos para realizar las
// siguientes acciones:
//      ➔ -crear una veterinaria.
//      ➔ -agregar una nueva mascota a la cola de atención de la veterinaria.
//      ➔ -agregar una nueva mascota a la cola de atención pero que sea la siguiente
//          en atender porque tiene la máxima prioridad.
//      ➔ -atender la próxima mascota de la cola.
//      ➔ -eliminar una mascota específica de la cola de atención dado que se retira.
//      ➔ -registrar una atención.
//      ➔ -buscar una atención dado el nombre de la mascota, el nombre del dueño y el
//          teléfono.
//      ➔ -modificar el diagnóstico de una determinada atención.
//      ➔ -modificar la fecha de la próxima visita de una determinada atención.
//      ➔ eliminar una determinada atención.
// Nota: para la fecha utilice lo implementado en el punto 3.

use super::ej03::Fecha;
use std::collections::VecDeque;

#[derive(Debug)]
enum EspecieAnimal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

impl EspecieAnimal {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (EspecieAnimal::PERRO, EspecieAnimal::PERRO)
            | (EspecieAnimal::GATO, EspecieAnimal::GATO)
            | (EspecieAnimal::CABALLO, EspecieAnimal::CABALLO)
            | (EspecieAnimal::OTROS, EspecieAnimal::OTROS) => true,
            _ => false,
        }
    }
    pub fn clone(&self) -> Self {
        match self {
            EspecieAnimal::PERRO => EspecieAnimal::PERRO,
            EspecieAnimal::GATO => EspecieAnimal::GATO,
            EspecieAnimal::CABALLO => EspecieAnimal::CABALLO,
            EspecieAnimal::OTROS => EspecieAnimal::OTROS,
        }
    }
}

#[derive(Debug)]
struct Mascota {
    nombre: String,
    edad: i32,
    tipo: EspecieAnimal,
    duenio: Duenio,
}

impl Mascota {
    pub fn equals(&self, other: &Self) -> bool {
        other.nombre == self.nombre
            && self.edad == other.edad
            && self.tipo.equals(&other.tipo)
            && self.duenio.equals(&other.duenio)
    }
    pub fn clone(&self) -> Self {
        Self {
            nombre: self.nombre.clone(),
            edad: self.edad.clone(),
            tipo: self.tipo.clone(),
            duenio: self.duenio.clone(),
        }
    }
}

#[derive(Debug)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String,
}

impl Duenio {
    pub fn equals(&self, other: &Self) -> bool {
        other.nombre == self.nombre
            && self.direccion == other.direccion
            && self.telefono == other.telefono
    }
    pub fn clone(&self) -> Self {
        Self {
            nombre: self.nombre.clone(),
            direccion: self.direccion.clone(),
            telefono: self.telefono.clone(),
        }
    }
}

#[derive(Debug)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha_proxima_visita: Option<Fecha>,
}

impl Atencion {
    pub fn equals(&self, other: &Self) -> bool {
        match (&self.fecha_proxima_visita, &other.fecha_proxima_visita) {
            (Some(f1), Some(f2)) => {
                self.mascota.equals(&other.mascota)
                    && self.diagnostico == other.diagnostico
                    && self.tratamiento == other.tratamiento
                    && f1.equals(f2)
            }
            (_, _) => {
                self.mascota.equals(&other.mascota)
                    && self.diagnostico == other.diagnostico
                    && self.tratamiento == other.tratamiento
            }
        }
    }
    pub fn clone(&self) -> Self {
        match &self.fecha_proxima_visita {
            Some(f) => Self {
                mascota: self.mascota.clone(),
                diagnostico: self.diagnostico.clone(),
                tratamiento: self.tratamiento.clone(),
                fecha_proxima_visita: Some(f.clone()),
            },
            _ => Self {
                mascota: self.mascota.clone(),
                diagnostico: self.diagnostico.clone(),
                tratamiento: self.tratamiento.clone(),
                fecha_proxima_visita: None,
            },
        }
    }
}

#[derive(Debug)]
struct Veterinaria {
    nombre: String,
    dirección: String,
    id: String,
    cola_atenciones: VecDeque<Mascota>,
    registro_atenciones: Vec<Atencion>,
}

impl Veterinaria {
    pub fn new(
        nombre: String,
        dirección: String,
        id: String,
        cola_atenciones: VecDeque<Mascota>,
        registro_atenciones: Vec<Atencion>,
    ) -> Veterinaria {
        Veterinaria {
            nombre,
            dirección,
            id,
            cola_atenciones,
            registro_atenciones,
        }
    }

    pub fn agregar_mascota_atras_cola(&mut self, mascota: Mascota) {
        self.cola_atenciones.push_back(mascota);
    }
    pub fn agregar_mascota_adelante_cola(&mut self, mascota: Mascota) {
        self.cola_atenciones.push_front(mascota);
    }

    pub fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        let prox_m = self.cola_atenciones.pop_front();
        if prox_m.is_some() {
            return prox_m;
        }
        None
    }

    fn get_pos_cola(&self, mascota: &Mascota) -> Option<usize> {
        for (i, m) in self.cola_atenciones.iter().enumerate() {
            if m.equals(mascota) {
                return Some(i);
            }
        }
        None
    }

    pub fn eliminar_mascota_cola(&mut self, mascota: &Mascota) {
        if let Some(pos) = self.get_pos_cola(mascota) {
            self.cola_atenciones.remove(pos);
        }
    }

    //=========================================
    //=========================================
    //=========================================

    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registro_atenciones.push(atencion);
    }

    pub fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_duenio: String,
        telefono: String,
    ) -> Option<&Atencion> {
        for atencion in &self.registro_atenciones {
            if atencion.mascota.nombre == nombre_mascota
                && atencion.mascota.duenio.nombre == nombre_duenio
                && atencion.mascota.duenio.telefono == telefono
            {
                let atencion_encontrada = atencion;
                return Some(&atencion_encontrada);
            }
        }
        None
    }

    fn get_pos_registro(&self, atencion: &Atencion) -> Option<usize> {
        for (i, a) in self.registro_atenciones.iter().enumerate() {
            if a.equals(atencion) {
                return Some(i);
            }
        }
        None
    }

    pub fn modificar_diagnostico_atencion(&mut self, atencion: &Atencion, nuevo_diag: String) {
        if let Some(atencion_index) = self.get_pos_registro(&atencion) {
            self.registro_atenciones[atencion_index].diagnostico = nuevo_diag;
        }
    }
    pub fn modificar_fecha_atencion(&mut self, atencion: &Atencion, nueva_fecha: Option<Fecha>) {
        if let Some(atencion_index) = self.get_pos_registro(&atencion) {
            self.registro_atenciones[atencion_index].fecha_proxima_visita = nueva_fecha;
        }
    }
    pub fn eliminar_atencion(&mut self, atencion: &Atencion) {
        if let Some(atencion_index) = self.get_pos_registro(&atencion) {
            self.registro_atenciones.remove(atencion_index);
        }
    }
}

// pub fn run() {
//     // Crear dueños
//     let duenio_1 = Duenio {
//         nombre: String::from("Alberto"),
//         direccion: String::from("Calle 48 164"),
//         telefono: String::from("221 304937"),
//     };

//     let duenio_2 = Duenio {
//         nombre: String::from("Maria"),
//         direccion: String::from("Avenida 10 102"),
//         telefono: String::from("222 587439"),
//     };

//     let duenio_3 = Duenio {
//         nombre: String::from("Pedro"),
//         direccion: String::from("Calle 32 75"),
//         telefono: String::from("223 901234"),
//     };

//     let duenio_4 = Duenio {
//         nombre: String::from("Luisa"),
//         direccion: String::from("Avenida 5 301"),
//         telefono: String::from("224 675821"),
//     };

//     // Crear mascotas
//     let mascota_1 = Mascota {
//         nombre: String::from("Rex"),
//         edad: 5,
//         tipo: EspecieAnimal::PERRO,
//         duenio: duenio_1.clone(),
//     };

//     let mascota_2 = Mascota {
//         nombre: String::from("Mishi"),
//         edad: 3,
//         tipo: EspecieAnimal::GATO,
//         duenio: duenio_2.clone(),
//     };

//     let mascota_3 = Mascota {
//         nombre: String::from("Firulais"),
//         edad: 2,
//         tipo: EspecieAnimal::PERRO,
//         duenio: duenio_3.clone(),
//     };

//     let mascota_4 = Mascota {
//         nombre: String::from("Garibaldi"),
//         edad: 1,
//         tipo: EspecieAnimal::CABALLO,
//         duenio: duenio_4.clone(),
//     };

//     let mascota_5 = Mascota {
//         nombre: String::from("Nemo"),
//         edad: 4,
//         tipo: EspecieAnimal::OTROS,
//         duenio: duenio_1.clone(),
//     };

//     // Crear veterinaria
//     let mut veterinaria = Veterinaria::new(
//         String::from("Clínica Veterinaria XYZ"),
//         String::from("Calle Principal 123"),
//         String::from("12345"),
//         VecDeque::new(), // La cola de atención comienza vacía
//         Vec::new(),      // El registro de atenciones comienza vacío
//     );

//     // Agregar mascotas a la cola de atención de la veterinaria
//     veterinaria.agregar_mascota_atras_cola(mascota_1.clone());
//     veterinaria.agregar_mascota_atras_cola(mascota_2.clone());
//     veterinaria.agregar_mascota_atras_cola(mascota_3.clone());
//     veterinaria.agregar_mascota_atras_cola(mascota_4.clone());
//     veterinaria.agregar_mascota_adelante_cola(mascota_5.clone());

//     // Eliminar una mascota específica de la cola de atención
//     veterinaria.eliminar_mascota_cola(&mascota_4);

//     // Atender la próxima mascota de la cola
//     let mascota_atendida = veterinaria.atender_proxima_mascota();
//     if let Some(mascota) = mascota_atendida {
//         println!("Se atendió a la mascota: {}", mascota.nombre);

//         // Registrar una atención
//         let atencion_registrada = Atencion {
//             mascota: mascota.clone(),
//             diagnostico: String::from("Resfriado"),
//             tratamiento: String::from("Antibióticos"),
//             fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
//         };
//         veterinaria.registrar_atencion(atencion_registrada);
//     }

//     // Atender la próxima mascota de la cola
//     let mascota_atendida = veterinaria.atender_proxima_mascota();
//     if let Some(mascota) = mascota_atendida {
//         println!("Se atendió a la mascota: {}", mascota.nombre);

//         // Registrar una atención
//         let atencion_registrada = Atencion {
//             mascota: mascota.clone(),
//             diagnostico: String::from("Resfriado"),
//             tratamiento: String::from("Antibióticos"),
//             fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
//         };
//         veterinaria.registrar_atencion(atencion_registrada);
//     }

//     // Buscar una atención
//     if let Some(atencion_buscada) = veterinaria.buscar_atencion(
//         mascota_5.nombre.clone(),
//         mascota_5.duenio.nombre.clone(),
//         mascota_5.duenio.telefono.clone(),
//     ) {
//         println!(
//             "Se encontró la atención de la mascota {}:",
//             mascota_5.nombre
//         );
//         println!("Diagnóstico: {}", atencion_buscada.diagnostico);
//         println!("Tratamiento: {}", atencion_buscada.tratamiento);
//         if let Some(fecha_prox_v) = &atencion_buscada.fecha_proxima_visita {
//             println!("Fecha próxima visita: {:#?}", fecha_prox_v);
//         } else {
//             println!("No hay fecha próxima visita registrada.");
//         }

//         veterinaria.modificar_fecha_atencion(
//             &Atencion {
//                 mascota: mascota_5.clone(),
//                 diagnostico: String::from("Resfriado"),
//                 tratamiento: String::from("Antibióticos"),
//                 fecha_proxima_visita: None,
//             },
//             Some(Fecha::new(20, 6, 2024)),
//         );
//     } else {
//         println!(
//             "No se encontró ninguna atención para la mascota {}.",
//             mascota_5.nombre
//         );
//     }

//     // Buscar una atención
//     if let Some(atencion_buscada) = veterinaria.buscar_atencion(
//         mascota_5.nombre.clone(),
//         mascota_5.duenio.nombre.clone(),
//         mascota_5.duenio.telefono.clone(),
//     ) {
//         println!(
//             "Se encontró la atención de la mascota {}:",
//             mascota_5.nombre
//         );
//         println!("Diagnóstico: {}", atencion_buscada.diagnostico);
//         println!("Tratamiento: {}", atencion_buscada.tratamiento);
//         if let Some(fecha_prox_v) = &atencion_buscada.fecha_proxima_visita {
//             println!("Fecha próxima visita: {:#?}", fecha_prox_v);
//         } else {
//             println!("No hay fecha próxima visita registrada.");
//         }
//         // Eliminar una atención
//         veterinaria.eliminar_atencion(&Atencion {
//             mascota: mascota_5.clone(),
//             diagnostico: String::from("Resfriado"),
//             tratamiento: String::from("Antibióticos"),
//             fecha_proxima_visita: None,
//         });
//     } else {
//         println!(
//             "No se encontró ninguna atención para la mascota {}.",
//             mascota_5.nombre
//         );
//     }

//     println!("{:#?}", veterinaria);
// }

mod tests {
    use super::*;

    #[test]
    fn test_creacion_mascotas_dueños() {
        // Verificar que se puedan crear instancias válidas de mascotas y dueños con los datos correctos.
        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456789"),
        };
        let mascota = Mascota {
            nombre: String::from("Fido"),
            edad: 3,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio.clone(),
        };

        assert_eq!(duenio.nombre, "Juan");
        assert_eq!(mascota.nombre, "Fido");
        assert_eq!(mascota.edad, 3);
        assert!(mascota.tipo.equals(&EspecieAnimal::PERRO));
        assert_eq!(mascota.duenio.nombre, "Juan");
        assert_eq!(mascota.duenio.direccion, "Calle 123");
        assert_eq!(mascota.duenio.telefono, "123456789");
    }

    #[test]
    fn test_agregar_mascotas_cola_atencion() {
        // Agregar varias mascotas a la cola de atención de la veterinaria y verificar que se hayan agregado correctamente y en el orden esperado.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio_1 = Duenio {
            nombre: String::from("Alberto"),
            direccion: String::from("Calle 48 164"),
            telefono: String::from("221 304937"),
        };

        let duenio_2 = Duenio {
            nombre: String::from("Maria"),
            direccion: String::from("Avenida 10 102"),
            telefono: String::from("222 587439"),
        };

        let mascota_1 = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio_1.clone(),
        };

        let mascota_2 = Mascota {
            nombre: String::from("Mishi"),
            edad: 3,
            tipo: EspecieAnimal::GATO,
            duenio: duenio_1.clone(),
        };

        let mascota_3 = Mascota {
            nombre: String::from("Garibaldi"),
            edad: 1,
            tipo: EspecieAnimal::CABALLO,
            duenio: duenio_2.clone(),
        };

        let mascota_4 = Mascota {
            nombre: String::from("Nemo"),
            edad: 4,
            tipo: EspecieAnimal::OTROS,
            duenio: duenio_1.clone(),
        };

        veterinaria.agregar_mascota_atras_cola(mascota_1.clone());
        veterinaria.agregar_mascota_atras_cola(mascota_2.clone());
        veterinaria.agregar_mascota_atras_cola(mascota_3.clone());
        veterinaria.agregar_mascota_adelante_cola(mascota_4.clone());

        assert_eq!(veterinaria.cola_atenciones.len(), 4);
        assert!(veterinaria.cola_atenciones[0].equals(&mascota_4));
        assert!(veterinaria.cola_atenciones[1].equals(&mascota_1));
        assert!(veterinaria.cola_atenciones[2].equals(&mascota_2));
        assert!(veterinaria.cola_atenciones[3].equals(&mascota_3));
    }

    #[test]
    fn test_atender_proxima_mascota() {
        // Agregar algunas mascotas a la cola de atención, realizar una atención y verificar que la próxima mascota en la cola sea atendida correctamente.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio_1 = Duenio {
            nombre: String::from("Alberto"),
            direccion: String::from("Calle 48 164"),
            telefono: String::from("221 304937"),
        };

        let duenio_2 = Duenio {
            nombre: String::from("Maria"),
            direccion: String::from("Avenida 10 102"),
            telefono: String::from("222 587439"),
        };

        let mascota_1 = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio_1.clone(),
        };

        let mascota_2 = Mascota {
            nombre: String::from("Mishi"),
            edad: 3,
            tipo: EspecieAnimal::GATO,
            duenio: duenio_1.clone(),
        };

        let mascota_3 = Mascota {
            nombre: String::from("Garibaldi"),
            edad: 1,
            tipo: EspecieAnimal::CABALLO,
            duenio: duenio_2.clone(),
        };

        let mascota_4 = Mascota {
            nombre: String::from("Nemo"),
            edad: 4,
            tipo: EspecieAnimal::OTROS,
            duenio: duenio_1.clone(),
        };

        veterinaria.agregar_mascota_atras_cola(mascota_1.clone());
        veterinaria.agregar_mascota_atras_cola(mascota_2.clone());
        veterinaria.agregar_mascota_atras_cola(mascota_3.clone());
        veterinaria.agregar_mascota_adelante_cola(mascota_4.clone());

        assert_eq!(veterinaria.cola_atenciones.len(), 4);
        if let Some(mascota) = veterinaria.atender_proxima_mascota() {
            assert!(mascota.equals(&mascota_4));
        }
        assert_eq!(veterinaria.cola_atenciones.len(), 3);
    }

    #[test]
    fn test_eliminar_mascota_especifica_cola() {
        // Agregar varias mascotas a la cola de atención, eliminar una mascota específica y verificar que la mascota correcta haya sido eliminada de la cola.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let mascota_1 = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: Duenio {
                nombre: String::from("Alberto"),
                direccion: String::from("Calle 48 164"),
                telefono: String::from("221 304937"),
            },
        };

        let mascota_2 = Mascota {
            nombre: String::from("Mishi"),
            edad: 3,
            tipo: EspecieAnimal::GATO,
            duenio: Duenio {
                nombre: String::from("Maria"),
                direccion: String::from("Avenida 10 102"),
                telefono: String::from("222 587439"),
            },
        };

        veterinaria.agregar_mascota_atras_cola(mascota_1.clone());
        veterinaria.agregar_mascota_atras_cola(mascota_2.clone());

        veterinaria.eliminar_mascota_cola(&mascota_1);
        assert_eq!(veterinaria.cola_atenciones.len(), 1);
    }

    // /\/\/\/\//\/\/\/\/\/\/\/
    // /\/\/\/\//\/\/\/\/\/\/\/
    // /\/\/\/\//\/\/\/\/\/\/\/

    #[test]
    fn test_registro_atencion() {
        // Registrar varias atenciones y verificar que se hayan registrado correctamente en el historial de atenciones de la veterinaria.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let mascota = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: Duenio {
                nombre: String::from("Alberto"),
                direccion: String::from("Calle 48 164"),
                telefono: String::from("221 304937"),
            },
        };

        let atencion_registrada = Atencion {
            mascota: mascota.clone(),
            diagnostico: String::from("Resfriado"),
            tratamiento: String::from("Antibióticos"),
            fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
        };

        veterinaria.registrar_atencion(atencion_registrada.clone());
        assert_eq!(veterinaria.registro_atenciones.len(), 1);
        assert!(veterinaria.registro_atenciones[0].equals(&atencion_registrada));
    }

    #[test]
    fn test_buscar_atencion() {
        // Realizar búsquedas de atención utilizando diferentes combinaciones de nombre de mascota, nombre de dueño y teléfono, y verificar que se encuentren las atenciones correctas.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456"),
        };

        let mascota = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio.clone(),
        };

        let atencion = Atencion {
            mascota: mascota.clone(),
            diagnostico: String::from("Resfriado"),
            tratamiento: String::from("Antibióticos"),
            fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
        };

        veterinaria.registrar_atencion(atencion.clone());

        if let Some(atencion_b) = veterinaria.buscar_atencion(
            mascota.nombre.clone(),
            duenio.nombre.clone(),
            duenio.telefono.clone(),
        ) {
            assert!(atencion_b.equals(&atencion));
        }
    }

    #[test]
    fn test_modificar_diagnostico_atencion() {
        // Modificar el diagnóstico de una atención existente y verificar que el cambio se haya realizado correctamente.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456"),
        };

        let mascota = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio.clone(),
        };

        let atencion = Atencion {
            mascota: mascota.clone(),
            diagnostico: String::from("Resfriado"),
            tratamiento: String::from("Antibióticos"),
            fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
        };

        veterinaria.registrar_atencion(atencion.clone());

        veterinaria.modificar_diagnostico_atencion(&atencion.clone(), "COVID".to_string());

        if let Some(atencion_b) = veterinaria.buscar_atencion(
            mascota.nombre.clone(),
            duenio.nombre.clone(),
            duenio.telefono.clone(),
        ) {
            assert_ne!(atencion_b.diagnostico, atencion.diagnostico);
        }
    }

    #[test]
    fn test_modificar_fecha_atencion() {
        // Modificar la fecha de próxima visita de una atención existente y verificar que el cambio se haya realizado correctamente.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456"),
        };

        let mascota = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio.clone(),
        };

        let atencion = Atencion {
            mascota: mascota.clone(),
            diagnostico: String::from("Resfriado"),
            tratamiento: String::from("Antibióticos"),
            fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
        };

        veterinaria.registrar_atencion(atencion.clone());

        veterinaria.modificar_fecha_atencion(&atencion.clone(), Some(Fecha::new(22, 8, 2024)));

        if let Some(atencion_b) = veterinaria.buscar_atencion(
            mascota.nombre.clone(),
            duenio.nombre.clone(),
            duenio.telefono.clone(),
        ) {
            assert!(!atencion_b.equals(&atencion));
        }
    }

    #[test]
    fn test_eliminar_atencion() {
        // Eliminar una atención del registro y verificar que haya sido eliminada correctamente.
        let mut veterinaria = Veterinaria::new(
            String::from("Clínica Veterinaria XYZ"),
            String::from("Calle Principal 123"),
            String::from("12345"),
            VecDeque::new(),
            Vec::new(),
        );

        let duenio = Duenio {
            nombre: String::from("Juan"),
            direccion: String::from("Calle 123"),
            telefono: String::from("123456"),
        };

        let mascota = Mascota {
            nombre: String::from("Rex"),
            edad: 5,
            tipo: EspecieAnimal::PERRO,
            duenio: duenio.clone(),
        };

        let atencion = Atencion {
            mascota: mascota.clone(),
            diagnostico: String::from("Resfriado"),
            tratamiento: String::from("Antibióticos"),
            fecha_proxima_visita: Some(Fecha::new(13, 5, 2024)),
        };

        veterinaria.registrar_atencion(atencion.clone());
        assert_eq!(veterinaria.registro_atenciones.len(), 1);
        veterinaria.eliminar_atencion(&atencion);
        assert_eq!(veterinaria.registro_atenciones.len(), 0);
    }
}
