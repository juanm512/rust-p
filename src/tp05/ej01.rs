// 1- En base al ejercicio 7 del tp#3 implemente lo siguiente:
// a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error
// propio con un mensaje de contexto.
// b- Haga todos los tests correspondientes para probar en profundidad los métodos
// que agregan un auto y eliminan un auto de la concesionaria , obteniendo el mayor
// porcentaje de coverage sobre el código que realiza las operaciones.
// c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
// almacenen en un archivo en formato JSON. Agregue y modifique lo que considere
// necesario para que:
// - Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a
// dicho archivo.
// - Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.
// No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que
// haga nueva funcionalidad..

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

// 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
// dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
// conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
// verde, azul, amarillo, blanco o negro.
// Para dichas estructuras implemente los siguientes métodos:
// ❖ ConcesionarioAuto:
// ➢ new: que pasando los parámetros correspondientes, crea un
// ConcesionarioAuto y lo retorna.
// ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
// la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
// no lo agrega y retorna false.
// ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
// ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
// ❖ Auto:
// ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
// retorna.
// ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
// ■ si es de color primario le aplica un recargo del 25%, sino le aplica un
// descuento del 10%.
// ■ si la marca es BMW le aplica un recargo del 15%-
// ■ si el año es menor a 2000 le aplica un descuento del 5%.

use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Write};

#[derive(Debug, Serialize, Deserialize)]
enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

impl Color {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::ROJO, Color::ROJO)
            | (Color::VERDE, Color::VERDE)
            | (Color::AZUL, Color::AZUL)
            | (Color::AMARILLO, Color::AMARILLO)
            | (Color::BLANCO, Color::BLANCO)
            | (Color::NEGRO, Color::NEGRO) => true,
            _ => false,
        }
    }
    pub fn clone(&self) -> Color {
        match self {
            Color::ROJO => Color::ROJO,
            Color::VERDE => Color::VERDE,
            Color::AZUL => Color::AZUL,
            Color::AMARILLO => Color::AMARILLO,
            Color::BLANCO => Color::BLANCO,
            Color::NEGRO => Color::NEGRO,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Auto {
    marca: String,
    modelo: String,
    año: i32,
    precio_bruto: f32,
    color: Color,
}

impl Auto {
    pub fn new(marca: String, modelo: String, año: i32, precio_bruto: f32, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        self.marca == other.marca
            && self.modelo == other.modelo
            && self.año == other.año
            && self.precio_bruto == other.precio_bruto
            && self.color.equals(&other.color)
    }
    pub fn clone(&self) -> Auto {
        Auto {
            marca: self.marca.clone(),
            modelo: self.modelo.clone(),
            año: self.año,
            precio_bruto: self.precio_bruto,
            color: self.color.clone(),
        }
    }

    pub fn calcular_precio(&self) -> f32 {
        let mut precio_total = self.precio_bruto;

        match self.color {
            Color::ROJO | Color::AZUL | Color::AMARILLO => precio_total += self.precio_bruto * 0.25,
            _ => precio_total -= self.precio_bruto * 0.1,
        }

        if self.marca == String::from("BMW") {
            precio_total += self.precio_bruto * 0.15
        }

        if self.año < 2000 {
            precio_total -= self.precio_bruto * 0.05
        }

        precio_total
    }
}

// error definicion
struct AgregarAutoError(i32);
impl Display for AgregarAutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error al agregar un auto, el concesionario esta a maxima capacidad: {}",
            self.0
        )
    }
}

struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    max_cap: i32,
    autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, max_cap: i32) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            max_cap,
            autos: Vec::new(),
        }
    }

    fn update_archivo(&self) {
        // aca guardar los cambios al archivo
        let autos_json = serde_json::to_string(&self.autos).unwrap();
        let mut f = File::create("./src/tp05/archivo_autos.json").unwrap();
        let result = f.write_all(&autos_json.as_bytes());
        match result {
            Ok(_) => println!("archivo_autos guardado correctamente"),
            Err(e) => println!("error: {}", e),
        }
    }

    pub fn agregar_auto(&mut self, new_auto: Auto) -> Result<Auto, AgregarAutoError> {
        if self.autos.len() == self.max_cap as usize {
            return Err(AgregarAutoError(self.max_cap));
        }
        let clon = new_auto.clone();
        self.autos.push(new_auto);
        self.update_archivo();

        Ok(clon)
    }

    pub fn eliminar_auto(&mut self, auto_buscado: &Auto) {
        for i in 0..self.autos.len() {
            if let Some(auto) = self.autos.get(i) {
                if auto.equals(auto_buscado) {
                    self.autos.remove(i);
                    self.update_archivo();
                }
            }
        }
    }

    pub fn buscar_auto(&self, auto_buscado: &Auto) -> Option<&Auto> {
        for auto in &self.autos {
            if auto.equals(auto_buscado) {
                return Some(auto);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_equals_success() {
        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );
        let auto2 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );

        assert_eq!(auto1.equals(&auto2), true);
    }

    #[test]
    fn test_auto_equals_fail() {
        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );
        let auto2 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::NEGRO,
        );

        assert_eq!(auto1.equals(&auto2), false);
    }

    #[test]
    fn test_auto_calcular_precio() {
        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20_000.0,
            Color::ROJO,
        );
        let auto2 = Auto::new(
            String::from("VW"),
            String::from("Gol"),
            1998,
            8_000.0,
            Color::AMARILLO,
        );
        let auto3 = Auto::new(
            String::from("BMW"),
            String::from("M50"),
            2015,
            15_000.0,
            Color::VERDE,
        );

        assert_eq!(auto1.calcular_precio(), 25000.0);
        assert_eq!(auto2.calcular_precio(), 9600.0);
        assert_eq!(auto3.calcular_precio(), 15750.0);
    }

    #[test]
    fn test_concesionario_agregar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            String::from("Concesionario XYZ"),
            String::from("Calle Principal"),
            2,
        );

        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );
        let auto2 = Auto::new(
            String::from("BMW"),
            String::from("X5"),
            2020,
            50000.0,
            Color::NEGRO,
        );
        let auto3 = Auto::new(
            String::from("Ford"),
            String::from("Fiesta"),
            2010,
            10000.0,
            Color::BLANCO,
        );

        let agregado1 = concesionario.agregar_auto(auto1);
        match agregado1 {
            Ok(ref auto) => println!("auto agregado correctamente: {:#?}", auto),
            Err(ref e) => println!("error: {}", e),
        }
        let agregado2 = concesionario.agregar_auto(auto2);
        match agregado2 {
            Ok(ref auto) => println!("auto agregado correctamente: {:#?}", auto),
            Err(ref e) => println!("error: {}", e),
        }
        let agregado3 = concesionario.agregar_auto(auto3);
        match agregado3 {
            Ok(ref auto) => println!("auto agregado correctamente: {:#?}", auto),
            Err(ref e) => println!("error: {}", e),
        }

        assert!(agregado1.is_ok());
        assert!(agregado2.is_ok());
        assert!(agregado3.is_err());
    }

    #[test]
    fn test_concesionario_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            String::from("Concesionario XYZ"),
            String::from("Calle Principal"),
            2,
        );

        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );
        let auto2 = Auto::new(
            String::from("BMW"),
            String::from("X5"),
            2020,
            50000.0,
            Color::NEGRO,
        );

        let _ = concesionario.agregar_auto(auto1.clone());
        let _ = concesionario.agregar_auto(auto2.clone());

        assert_eq!(concesionario.autos.len(), 2);

        concesionario.eliminar_auto(&auto1);

        assert_eq!(concesionario.autos.len(), 1);
        assert_eq!(concesionario.autos[0].equals(&auto1), false);
    }

    #[test]
    fn test_concesionario_buscar_auto_success() {
        let mut concesionario = ConcesionarioAuto::new(
            String::from("Concesionario XYZ"),
            String::from("Calle Principal"),
            6,
        );
        let auto1 = Auto::new(
            String::from("Toyota"),
            String::from("Corolla"),
            2018,
            20000.0,
            Color::AZUL,
        );
        let auto2 = Auto::new(
            String::from("BMW"),
            String::from("X5"),
            2020,
            34000.0,
            Color::NEGRO,
        );
        let auto3 = Auto::new(
            String::from("12DSA"),
            String::from("ASD"),
            2000,
            50000.0,
            Color::VERDE,
        );
        let auto4 = Auto::new(
            String::from("QWE"),
            String::from("L P2"),
            2010,
            52000.0,
            Color::AMARILLO,
        );
        let auto5 = Auto::new(
            String::from("NUFDASI"),
            String::from("DSA"),
            2002,
            26400.0,
            Color::BLANCO,
        );
        let auto6 = Auto::new(
            String::from("4THVC"),
            String::from("DA"),
            1984,
            20000.0,
            Color::ROJO,
        );

        let _ = concesionario.agregar_auto(auto3.clone());
        let _ = concesionario.agregar_auto(auto5.clone());
        let _ = concesionario.agregar_auto(auto4.clone());
        let _ = concesionario.agregar_auto(auto6.clone());
        let _ = concesionario.agregar_auto(auto1.clone());
        let _ = concesionario.agregar_auto(auto2.clone());

        if let Some(auto_encontrado) = concesionario.buscar_auto(&auto1) {
            assert!(auto_encontrado.equals(&auto1));
        }
        let auto3 = Auto::new(
            String::from("Ford"),
            String::from("Fiesta"),
            2010,
            10000.0,
            Color::BLANCO,
        );
        let auto_encontrado = concesionario.buscar_auto(&auto3);
        assert!(auto_encontrado.is_none());
    }
}
