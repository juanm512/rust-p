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
use chrono::prelude::*;
use std::collections::{HashMap, LinkedList};

use super::ej03::Fecha;

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn new(
        nombre: String,
        direccion: String,
        libros_disponibles: HashMap<String, u32>,
        prestamos_efectuados: Vec<Prestamo>,
    ) -> Self {
        Self {
            nombre,
            direccion,
            libros_disponibles,
            prestamos_efectuados,
        }
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
        }
    }

    // ➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1 la cantidad de copias del libro a disposición para ser prestado.
    pub fn incrementar_cantidad_de_copias(&mut self, libro_isbn: String) {
        if let Some(value) = self.libros_disponibles.get_mut(&libro_isbn) {
            *value += 1;
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

    fn cambiar_estado_prestamo(&mut self, prestamo_b: Prestamo) {
        for prestamo in &mut self.prestamos_efectuados {
            if prestamo.equals(&prestamo_b) {
                match prestamo.estado {
                    EstadoPrestamo::PRESTADO => prestamo.estado = EstadoPrestamo::DEVUELTO,
                    EstadoPrestamo::DEVUELTO => prestamo.estado = EstadoPrestamo::PRESTADO,
                }
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
            self.cambiar_estado_prestamo(prestamo.clone());
        }
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            self.cambiar_fecha_devolucion_prestamo(prestamo.clone());
        }
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            self.incrementar_cantidad_de_copias(prestamo.libro.isbn.clone());
        }
    }
}

//=========================================================================================

pub fn run() {}

mod tests {
    use super::*;

    #[test]
    fn test_obtener_cantidad_de_copias() {
        let biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::from([("isbn1".to_string(), 5), ("isbn2".to_string(), 3)]),
            Vec::new(),
        );

        assert_eq!(
            biblioteca.obtener_cantidad_de_copias(&Libro::new(
                "isbn1".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            )),
            5
        );

        assert_eq!(
            biblioteca.obtener_cantidad_de_copias(&Libro::new(
                "isbn3".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            )),
            0
        );
    }

    #[test]
    fn test_decrementar_cantidad_de_copias() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::from([("isbn1".to_string(), 5), ("isbn2".to_string(), 3)]),
            Vec::new(),
        );

        biblioteca.decrementar_cantidad_de_copias("isbn1".to_string());
        assert_eq!(
            biblioteca.obtener_cantidad_de_copias(&Libro::new(
                "isbn1".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            )),
            4
        );
    }

    #[test]
    fn test_incrementar_cantidad_de_copias() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::from([("isbn1".to_string(), 5), ("isbn2".to_string(), 3)]),
            Vec::new(),
        );

        biblioteca.incrementar_cantidad_de_copias("isbn1".to_string());
        assert_eq!(
            biblioteca.obtener_cantidad_de_copias(&Libro::new(
                "isbn1".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            )),
            6
        );
    }

    #[test]
    fn test_contar_cantidad_de_prestamos_cliente() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::new(),
            Vec::new(),
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            Libro::new(
                "isbn1".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            ),
            cliente.clone(),
            Fecha::new(1, 1, 2022),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            Libro::new(
                "isbn2".to_string(),
                "titulo".to_string(),
                "autor".to_string(),
                100,
                Genero::NOVELA,
            ),
            cliente.clone(),
            Fecha::new(1, 1, 2022),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        assert_eq!(biblioteca.contar_cantidad_de_prestamos_cliente(&cliente), 2);
    }

    #[test]
    fn test_realizar_prestamo() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::from([("isbn1".to_string(), 5)]),
            Vec::new(),
        );

        let libro = Libro::new(
            "isbn1".to_string(),
            "titulo".to_string(),
            "autor".to_string(),
            100,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        assert!(biblioteca.realizar_prestamo(&libro, &cliente));
        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 4);
    }

    #[test]
    fn test_ver_los_prestamos_a_vencer() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            "isbn1".to_string(),
            "titulo".to_string(),
            "autor".to_string(),
            100,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            libro.clone(),
            cliente.clone(),
            Fecha::new(15, 5, 2024),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        let lista = biblioteca.ver_los_prestamos_a_vencer(7);
        assert_eq!(lista.len(), 1);
    }

    #[test]
    fn test_ver_los_prestamos_vencidos() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            "isbn1".to_string(),
            "titulo".to_string(),
            "autor".to_string(),
            100,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            libro.clone(),
            cliente.clone(),
            Fecha::new(1, 1, 2022),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        let lista = biblioteca.ver_los_prestamos_vencidos();
        assert_eq!(lista.len(), 1);
    }

    #[test]
    fn test_buscar_prestamo() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::new(),
            Vec::new(),
        );

        let libro = Libro::new(
            "isbn1".to_string(),
            "titulo".to_string(),
            "autor".to_string(),
            100,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        biblioteca.prestamos_efectuados.push(Prestamo::new(
            libro.clone(),
            cliente.clone(),
            Fecha::new(1, 1, 2022),
            None,
            EstadoPrestamo::PRESTADO,
        ));

        if let Some(prestamo) = biblioteca.buscar_prestamo(&libro, &cliente) {
            assert_eq!(prestamo.libro.isbn, libro.isbn);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca".to_string(),
            "Direccion".to_string(),
            HashMap::from([("isbn1".to_string(), 5)]),
            Vec::new(),
        );

        let libro = Libro::new(
            "isbn1".to_string(),
            "titulo".to_string(),
            "autor".to_string(),
            100,
            Genero::NOVELA,
        );

        let cliente = Cliente::new(
            "nombre".to_string(),
            "telefono".to_string(),
            "email".to_string(),
        );

        biblioteca.realizar_prestamo(&libro, &cliente);
        biblioteca.devolver_libro(&libro, &cliente);

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro), 5);
    }
}
