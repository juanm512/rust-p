// 4- En base al ejercicio 10 del tp#3 implemente lo siguiente:
// a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
// de por lo menos 90%
// b- Tanto los libros con sus copias como la administración de préstamos se realizan
// sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
// así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
// haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
// manteniendo un coverage de al menos 90%.

// 10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
// biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
// prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
// la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
// cada libro se conoce el isbn, el título, autor, número de páginas, género(novela, infantil,
// técnico, otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de
// vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
// préstamo. Del cliente se conoce el nombre, teléfono y dirección de correo electrónico.

// Implemente los métodos necesarios para realizar las siguientes acciones:
//      ➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de
//          copias a disposición que hay para prestar de dicho libro.
//      ➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
//          la cantidad de copias de libros a disposición para prestar.
//      ➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1
//          la cantidad de copias del libro a disposición para ser prestado.
//      ➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
//          “en préstamo” de un determinado cliente.

//      ➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
//          para un determinado cliente cumpliendo con lo siguiente
//              ◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
//              ◆ haya al menos una copia disponible en el registro de copias a
//                  disposición.
//                 De ser así descuenta 1 en el registro de “copias a disposición” y
//                  retorna true, si no cumple con alguna de las condiciones retorna false.
//      ➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
//          vencer el los próximos días, el valor de días es pasado por parámetro.
//      ➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
//          préstamos” donde la fecha de vencimiento es menor a la fecha actual.

//      ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
//          existe.
//      ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
//          estado “devuelto”, se registra la fecha de devolución y se incrementa la
//          cantidad de libros en 1 del libro devuelto en el registro de copias a
//          disposición.
//  Nota: para la fecha utilice lo implementado en el punto 3

//=========================================================================================
use crate::tp03::ej03::Fecha;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::collections::{HashMap, LinkedList};
use std::io::Read;
use std::{fmt::Display, fs::File, io::Write};

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
#[derive(Debug, Serialize, Deserialize)]
enum Genero {
    NOVELA,
    INFANTIL,
    TECNICO,
    OTROS,
}

impl Genero {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Genero::NOVELA, Genero::NOVELA)
            | (Genero::INFANTIL, Genero::INFANTIL)
            | (Genero::TECNICO, Genero::TECNICO)
            | (Genero::OTROS, Genero::OTROS) => true,
            _ => false,
        }
    }
    pub fn clone(&self) -> Self {
        match self {
            Genero::NOVELA => Genero::NOVELA,
            Genero::INFANTIL => Genero::INFANTIL,
            Genero::TECNICO => Genero::TECNICO,
            Genero::OTROS => Genero::OTROS,
        }
    }
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Serialize, Deserialize)]
enum EstadoPrestamo {
    PRESTADO,
    DEVUELTO,
}
impl EstadoPrestamo {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (EstadoPrestamo::PRESTADO, EstadoPrestamo::PRESTADO)
            | (EstadoPrestamo::DEVUELTO, EstadoPrestamo::DEVUELTO) => true,
            _ => false,
        }
    }
    pub fn clone(&self) -> Self {
        match self {
            EstadoPrestamo::PRESTADO => EstadoPrestamo::PRESTADO,
            EstadoPrestamo::DEVUELTO => EstadoPrestamo::DEVUELTO,
        }
    }
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Serialize, Deserialize)]
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    num_paginas: u16,
    genero: Genero,
}

impl Libro {
    pub fn new(
        isbn: String,
        titulo: String,
        autor: String,
        num_paginas: u16,
        genero: Genero,
    ) -> Self {
        Self {
            isbn,
            titulo,
            autor,
            num_paginas,
            genero,
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        other.isbn == self.isbn
            && self.titulo == other.titulo
            && self.autor == other.autor
            && self.num_paginas == other.num_paginas
            && self.genero.equals(&other.genero)
    }
    pub fn clone(&self) -> Self {
        Self {
            isbn: self.isbn.clone(),
            titulo: self.titulo.clone(),
            autor: self.autor.clone(),
            num_paginas: self.num_paginas.clone(),
            genero: self.genero.clone(),
        }
    }
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Serialize, Deserialize)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}
impl Cliente {
    pub fn new(nombre: String, telefono: String, email: String) -> Self {
        Self {
            nombre,
            telefono,
            email,
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        other.nombre == self.nombre && self.telefono == other.telefono && self.email == other.email
    }
    pub fn clone(&self) -> Self {
        Self {
            nombre: self.nombre.clone(),
            telefono: self.telefono.clone(),
            email: self.email.clone(),
        }
    }
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Serialize, Deserialize)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: EstadoPrestamo,
}

impl Prestamo {
    pub fn new(
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
        fecha_devolucion: Option<Fecha>,
        estado: EstadoPrestamo,
    ) -> Self {
        Self {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion,
            estado,
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        match (&self.fecha_devolucion, &other.fecha_devolucion) {
            (Some(fecha1), Some(fecha2)) => {
                other.libro.equals(&self.libro)
                    && self.cliente.equals(&other.cliente)
                    && self.fecha_vencimiento.equals(&other.fecha_vencimiento)
                    && fecha1.equals(&fecha2)
                    && self.estado.equals(&other.estado)
            }
            (None, None) => {
                other.libro.equals(&self.libro)
                    && self.cliente.equals(&other.cliente)
                    && self.fecha_vencimiento.equals(&other.fecha_vencimiento)
                    && self.estado.equals(&other.estado)
            }
            (_, _) => false,
        }
    }
    pub fn clone(&self) -> Self {
        match &self.fecha_devolucion {
            Some(fecha) => Self {
                libro: self.libro.clone(),
                cliente: self.cliente.clone(),
                fecha_vencimiento: self.fecha_vencimiento.clone(),
                fecha_devolucion: Some(fecha.clone()),
                estado: self.estado.clone(),
            },
            None => Self {
                libro: self.libro.clone(),
                cliente: self.cliente.clone(),
                fecha_vencimiento: self.fecha_vencimiento.clone(),
                fecha_devolucion: None,
                estado: self.estado.clone(),
            },
        }
    }
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: HashMap<String, u32>,
    prestamos_efectuados: Vec<Prestamo>,
}
impl Biblioteca {
    fn read_archivos() -> (HashMap<String, u32>, Vec<Prestamo>) {
        let mut buf_libros = String::new();
        let mut buf_prestamos = String::new();

        let mut f_libros_disponibles = File::open("./src/tp05/archivo_libros_disponibles.json")
            .unwrap_or_else(|_| {
                eprintln!("Error al abrir archivo_libros_disponibles.json");
                File::create("./src/tp05/archivo_libros_disponibles.json").unwrap()
            });

        let mut f_prestamos_efectuados = File::open("./src/tp05/archivo_prestamos_efectuados.json")
            .unwrap_or_else(|_| {
                eprintln!("Error al abrir archivo_prestamos_efectuados.json");
                File::create("./src/tp05/archivo_prestamos_efectuados.json").unwrap()
            });

        let _ = f_prestamos_efectuados.read_to_string(&mut buf_prestamos);
        let result_prestamos_efectuados: SerdeResult<Vec<Prestamo>> =
            serde_json::from_str(&buf_prestamos);

        let _ = f_libros_disponibles.read_to_string(&mut buf_libros);
        let result_libros_disponibles: SerdeResult<HashMap<String, u32>> =
            serde_json::from_str(&buf_libros);

        let libros_disponibles = match result_libros_disponibles {
            Ok(hashmap) => hashmap,
            Err(e) => {
                println!("Error al parsear archivo_libros_disponibles.json: {}", e);
                HashMap::new()
            }
        };

        let prestamos_efectuados = match result_prestamos_efectuados {
            Ok(vec) => vec,
            Err(e) => {
                println!("Error al parsear archivo_prestamos_efectuados.json: {}", e);
                Vec::new()
            }
        };

        (libros_disponibles, prestamos_efectuados)
    }

    fn update_archivo_prestamos_efectuados(&self) {
        // aca guardar los cambios al archivo
        let prestamos_efectuados_json = serde_json::to_string(&self.prestamos_efectuados).unwrap();
        let mut f = File::create("./src/tp05/archivo_prestamos_efectuados.json").unwrap();
        let result = f.write_all(&prestamos_efectuados_json.as_bytes());
        match result {
            Ok(_) => println!("prestamos_efectuados guardado correctamente"),
            Err(e) => println!("error: {}", e),
        }
    }
    fn update_archivo_libros_disponibles(&self) {
        // aca guardar los cambios al archivo
        let libros_disponibles_json = serde_json::to_string(&self.libros_disponibles).unwrap();
        let mut f = File::create("./src/tp05/archivo_libros_disponibles.json").unwrap();
        let result = f.write_all(&libros_disponibles_json.as_bytes());
        match result {
            Ok(_) => println!("libros_disponibles guardado correctamente"),
            Err(e) => println!("error: {}", e),
        }
    }

    pub fn new(
        nombre: String,
        direccion: String,
        mut libros_disponibles: HashMap<String, u32>,
        mut prestamos_efectuados: Vec<Prestamo>,
    ) -> Self {
        let result = Self::read_archivos();
        let libros_d = result.0;

        libros_d.iter().for_each(|l_d| {
            libros_disponibles.insert(l_d.0.to_string(), *l_d.1);
        });

        let prestamos_e = result.1;

        prestamos_e.iter().for_each(|prestamo| {
            prestamos_efectuados.push(prestamo.clone());
        });

        return Self {
            nombre,
            direccion,
            libros_disponibles,
            prestamos_efectuados,
        };
    }

    pub fn agregar_datos_libros(&mut self, libro_b: String, copias: u32) {
        if let Some(value) = self.libros_disponibles.get_mut(&libro_b) {
            *value += copias;
            self.update_archivo_libros_disponibles();
            return;
        }
        self.libros_disponibles.insert(libro_b, copias);
        self.update_archivo_libros_disponibles();
    }

    // ➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de copias a disposición que hay para prestar de dicho libro.
    pub fn obtener_cantidad_de_copias(&self, libro_b: &Libro) -> u32 {
        if let Some(value) = self.libros_disponibles.get(&libro_b.isbn) {
            return *value;
        }
        0
    }

    // ➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1 la cantidad de copias de libros a disposición para prestar.
    pub fn decrementar_cantidad_de_copias(&mut self, libro_isbn: String) {
        if let Some(value) = self.libros_disponibles.get_mut(&libro_isbn) {
            *value -= 1;
            self.update_archivo_libros_disponibles();
        }
    }

    // ➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1 la cantidad de copias del libro a disposición para ser prestado.
    pub fn incrementar_cantidad_de_copias(&mut self, libro_isbn: String) {
        if let Some(value) = self.libros_disponibles.get_mut(&libro_isbn) {
            *value += 1;
            self.update_archivo_libros_disponibles();
        }
    }

    // ➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado “en préstamo” de un determinado cliente.
    pub fn contar_cantidad_de_prestamos_cliente(&mut self, cliente: &Cliente) -> u32 {
        let mut cant = 0;

        for prestamo in &self.prestamos_efectuados {
            if prestamo.cliente.equals(cliente) && prestamo.estado.equals(&EstadoPrestamo::PRESTADO)
            {
                cant += 1;
            }
        }

        cant
    }

    //  ➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro para un determinado cliente cumpliendo con lo siguiente:
    //              ◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
    //              ◆ haya al menos una copia disponible en el registro de copias a
    //                  disposición.
    //                 De ser así descuenta 1 en el registro de “copias a disposición” y
    //                  retorna true, si no cumple con alguna de las condiciones retorna false.
    pub fn realizar_prestamo(&mut self, libro: &Libro, cliente: &Cliente) -> bool {
        if self.contar_cantidad_de_prestamos_cliente(cliente) <= 5
            && self.obtener_cantidad_de_copias(libro) > 0
        {
            let local: DateTime<Local> = Local::now();
            let mut fecha_vencimiento =
                Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());
            fecha_vencimiento.sumar_dias(7);

            self.prestamos_efectuados.push(Prestamo::new(
                libro.clone(),
                cliente.clone(),
                fecha_vencimiento,
                None,
                EstadoPrestamo::PRESTADO,
            ));
            {
                self.decrementar_cantidad_de_copias(libro.isbn.clone());
            }
            self.update_archivo_prestamos_efectuados();
            return true;
        }
        false
    }

    //  ➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a vencer el los próximos días, el valor de días es pasado por parámetro.
    pub fn ver_los_prestamos_a_vencer(&mut self, dias: u32) -> LinkedList<Prestamo> {
        let mut lista: LinkedList<Prestamo> = LinkedList::new();
        let local: DateTime<Local> = Local::now();
        let fecha_actual = Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());
        let mut fecha_proxima =
            Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());
        fecha_proxima.sumar_dias(dias);

        for prestamo in &self.prestamos_efectuados {
            if prestamo.fecha_vencimiento.es_mayor(&fecha_actual)
                && fecha_proxima.es_mayor(&prestamo.fecha_vencimiento)
            {
                lista.push_back(prestamo.clone())
            }
        }

        lista
    }

    //  ➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en préstamos” donde la fecha de vencimiento es menor a la fecha actual.
    pub fn ver_los_prestamos_vencidos(&mut self) -> LinkedList<Prestamo> {
        let mut lista: LinkedList<Prestamo> = LinkedList::new();
        let local: DateTime<Local> = Local::now();
        let fecha_actual = Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());

        for prestamo in &self.prestamos_efectuados {
            if prestamo.estado.equals(&EstadoPrestamo::PRESTADO)
                && fecha_actual.es_mayor(&prestamo.fecha_vencimiento)
            {
                lista.push_back(prestamo.clone())
            }
        }

        lista
    }

    //  ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si existe.
    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        for prestamo in &self.prestamos_efectuados {
            if prestamo.cliente.equals(cliente) && prestamo.libro.equals(libro) {
                return Some(prestamo);
            }
        }
        None
    }

    fn cambiar_estado_prestamo(&mut self, prestamo_b: Prestamo, nuevo_estado: EstadoPrestamo) {
        for prestamo in &mut self.prestamos_efectuados {
            if prestamo.equals(&prestamo_b) {
                prestamo.estado = nuevo_estado.clone();
            }
        }
    }
    fn cambiar_fecha_devolucion_prestamo(&mut self, prestamo_b: Prestamo) {
        let local: DateTime<Local> = Local::now();
        for prestamo in &mut self.prestamos_efectuados {
            if prestamo.equals(&prestamo_b) {
                prestamo.fecha_devolucion = Some(Fecha::new(
                    local.day(),
                    local.month(),
                    local.year().try_into().unwrap(),
                ))
            }
        }
    }

    //  ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al estado “devuelto”, se registra la fecha de devolución y se incrementa la
    //      cantidad de libros en 1 del libro devuelto en el registro de copias a disposición.
    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente) {
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            self.cambiar_estado_prestamo(prestamo.clone(), EstadoPrestamo::DEVUELTO);
        }
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            self.cambiar_fecha_devolucion_prestamo(prestamo.clone());
        }
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            self.incrementar_cantidad_de_copias(prestamo.libro.isbn.clone());
        }
        self.update_archivo_prestamos_efectuados();
    }
}

//=========================================================================================
//=========================================================================================
//=========================================================================================
//=========================================================================================
//=========================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use serde_json;
    use std::fs::{remove_file, File};
    use std::io::Write;

    fn iniciar_datos() {
        let mut f_libros_disponibles =
            File::create("./src/tp05/archivo_libros_disponibles.json").unwrap();
        let libros = r#"{"1234567890": 3, "0987654321": 2}"#;
        f_libros_disponibles.write_all(libros.as_bytes()).unwrap();

        let mut f_prestamos_efectuados =
            File::create("./src/tp05/archivo_prestamos_efectuados.json").unwrap();
        let prestamos = r#"[]"#;
        f_prestamos_efectuados
            .write_all(prestamos.as_bytes())
            .unwrap();
    }

    fn resetear() {
        let _ = remove_file("./src/tp05/archivo_libros_disponibles.json");
        let _ = remove_file("./src/tp05/archivo_prestamos_efectuados.json");
    }

    #[test]
    fn test_biblioteca_new() {
        iniciar_datos();

        let biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        assert_eq!(biblioteca.nombre, "Biblioteca Central");
        assert_eq!(biblioteca.direccion, "Av. Principal 123");
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get("1234567890")
                .cloned()
                .unwrap_or(0),
            3
        );
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get("0987654321")
                .cloned()
                .unwrap_or(0),
            2
        );
        assert_eq!(biblioteca.prestamos_efectuados.len(), 0);

        resetear();
    }

    #[test]
    fn test_agregar_datos_libros() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        biblioteca.agregar_datos_libros(String::from("1234567890"), 2);
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get("1234567890")
                .cloned()
                .unwrap_or(0),
            5
        );

        resetear();
    }

    #[test]
    fn test_obtener_cantidad_de_copias() {
        iniciar_datos();

        let biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::TECNICO,
        );

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 3);

        resetear();
    }

    #[test]
    fn test_decrementar_cantidad_de_copias() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        biblioteca.decrementar_cantidad_de_copias(String::from("1234567890"));
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get("1234567890")
                .cloned()
                .unwrap_or(0),
            2
        );

        resetear();
    }

    #[test]
    fn test_incrementar_cantidad_de_copias() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        biblioteca.incrementar_cantidad_de_copias(String::from("1234567890"));
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get("1234567890")
                .cloned()
                .unwrap_or(0),
            4
        );

        resetear();
    }

    #[test]
    fn test_contar_cantidad_de_prestamos_cliente() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::NOVELA,
        );

        biblioteca.realizar_prestamo(&libro, &cliente);

        assert_eq!(biblioteca.contar_cantidad_de_prestamos_cliente(&cliente), 1);

        resetear();
    }

    #[test]
    fn test_realizar_prestamo() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::OTROS,
        );

        let result = biblioteca.realizar_prestamo(&libro, &cliente);
        assert!(result);
        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 2);

        resetear();
    }

    #[test]
    fn test_devolver_libro() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::NOVELA,
        );

        biblioteca.realizar_prestamo(&libro, &cliente);
        biblioteca.devolver_libro(&libro, &cliente);

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 3);

        resetear();
    }

    #[test]
    fn test_ver_los_prestamos_a_vencer() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let local: DateTime<Local> = Local::now();
        let mut fecha_vencimiento =
            Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());
        fecha_vencimiento.sumar_dias(3);

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            libro.clone(),
            cliente.clone(),
            fecha_vencimiento.clone(),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        let prestamos_vencer = biblioteca.ver_los_prestamos_a_vencer(5);
        assert_eq!(prestamos_vencer.len(), 1);
        assert_eq!(prestamos_vencer.front().unwrap().libro.isbn, libro.isbn);

        resetear();
    }

    #[test]
    fn test_ver_los_prestamos_vencidos() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let local: DateTime<Local> = Local::now();
        let mut fecha_vencimiento =
            Fecha::new(local.day(), local.month(), local.year().try_into().unwrap());
        fecha_vencimiento.restar_dias(3);

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            libro.clone(),
            cliente.clone(),
            fecha_vencimiento.clone(),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        let prestamos_vencidos = biblioteca.ver_los_prestamos_vencidos();
        assert_eq!(prestamos_vencidos.len(), 1);
        assert_eq!(prestamos_vencidos.front().unwrap().libro.isbn, libro.isbn);

        resetear();
    }

    #[test]
    fn test_read_archivos_open_error() {
        let _ = remove_file("./src/tp05/archivo_libros_disponibles.json");
        let _ = remove_file("./src/tp05/archivo_prestamos_efectuados.json");

        let (libros_disponibles, prestamos_efectuados) = Biblioteca::read_archivos();

        assert!(libros_disponibles.is_empty());
        assert!(prestamos_efectuados.is_empty());
    }

    #[test]
    fn test_read_archivos_parse_error() {
        let mut f_libros_disponibles =
            File::create("./src/tp05/archivo_libros_disponibles.json").unwrap();
        let libros_corruptos = r#"{"1234567890": "tres", "0987654321": "dos"}"#;
        f_libros_disponibles
            .write_all(libros_corruptos.as_bytes())
            .unwrap();

        let mut f_prestamos_efectuados =
            File::create("./src/tp05/archivo_prestamos_efectuados.json").unwrap();
        let prestamos_corruptos = r#"{"prestamos": [1, 2, 3]}"#;
        f_prestamos_efectuados
            .write_all(prestamos_corruptos.as_bytes())
            .unwrap();

        let (libros_disponibles, prestamos_efectuados) = Biblioteca::read_archivos();

        assert!(libros_disponibles.is_empty());
        assert!(prestamos_efectuados.is_empty());
    }

    #[test]
    fn test_obtener_cantidad_de_copias_libro_inexistente() {
        iniciar_datos();

        let biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            String::from("1111111111"),
            String::from("Libro Inexistente"),
            String::from("Autor Desconocido"),
            100,
            Genero::OTROS,
        );

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 0);

        resetear();
    }

    #[test]
    fn test_agregar_datos_libros_nuevo() {
        iniciar_datos();

        let mut biblioteca = Biblioteca::new(
            String::from("Biblioteca Central"),
            String::from("Av. Principal 123"),
            HashMap::new(),
            Vec::new(),
        );

        let nuevo_libro = String::from("1111111111");
        biblioteca.agregar_datos_libros(nuevo_libro.clone(), 2);
        assert_eq!(
            biblioteca
                .libros_disponibles
                .get(&nuevo_libro)
                .cloned()
                .unwrap_or(0),
            2
        );

        resetear();
    }

    #[test]
    fn test_prestamo_equals_y_clone_con_fecha_devolucion() {
        let libro = Libro::new(
            String::from("1234567890"),
            String::from("El libro"),
            String::from("El autor"),
            300,
            Genero::INFANTIL,
        );

        let cliente = Cliente::new(
            String::from("Juan Perez"),
            String::from("123456789"),
            String::from("juan.perez@example.com"),
        );

        let fecha_vencimiento = Fecha::new(1, 1, 2023);
        let fecha_devolucion = Some(Fecha::new(8, 1, 2023));

        let prestamo1 = Prestamo::new(
            libro.clone(),
            cliente.clone(),
            fecha_vencimiento.clone(),
            fecha_devolucion,
            EstadoPrestamo::PRESTADO,
        );

        let prestamo2 = prestamo1.clone();

        assert!(prestamo1.equals(&prestamo2));
        assert!(prestamo1.clone().equals(&prestamo2));
    }
}
