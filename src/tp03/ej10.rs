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

use std::collections::HashMap;

use super::ej03::Fecha;

// género(novela, infantil,
// técnico, otros)
enum Genero {
    NOVELA,
    INFANTIL,
    TECNICO,
    OTROS,
}

impl Genero {
    pub fn equals() {}
    pub fn clone() {}
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

// libro se conoce el isbn, el título, autor, número de páginas, género(novela, infantil,
// técnico, otros)
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
    pub fn equals() {}
    pub fn clone() {}
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

// cliente se conoce el nombre, teléfono y dirección de correo electrónico
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
    pub fn equals() {}
    pub fn clone() {}
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

// registrar un préstamo se requiere el libro, el cliente, la fecha de
// vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
// préstamo

enum EstadoPrestamo {
    PRESTADO,
    DEVUELTO,
}
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Fecha,
    estado: EstadoPrestamo,
}
impl Prestamo {
    pub fn new(
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
        fecha_devolucion: Fecha,
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
    pub fn equals() {}
    pub fn clone() {}
}

//\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\

// biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
// prestar y los préstamos efectuados
// libros a disposición es un registro donde se indica
// la cantidad de ejemplares que tiene a disposición para prestar de determinado libro

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
    pub fn equals() {}
    pub fn clone() {}
}

//=========================================================================================

pub fn run() {}

#[cfg(test)]
mod tests {
    use super::*;
}
