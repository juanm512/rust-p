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

#[derive(Debug)]
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

#[derive(Debug)]
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
            precio_total -= self.precio_bruto * 0.5
        }

        precio_total
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

    pub fn agregar_auto(&mut self, new_auto: Auto) -> bool {
        if self.autos.len() == self.max_cap as usize {
            return false;
        }

        self.autos.push(new_auto);
        true
    }

    pub fn eliminar_auto(&mut self, auto_buscado: &Auto) {
        for i in 0..self.autos.len() {
            if let Some(auto) = self.autos.get(i) {
                if auto.equals(auto_buscado) {
                    self.autos.remove(i);
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

pub fn run() {
    // Crear algunos autos de ejemplo
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

    // Crear un concesionario
    let mut concesionario = ConcesionarioAuto::new(
        String::from("Concesionario XYZ"),
        String::from("Calle Principal"),
        3,
    );

    // Agregar autos al concesionario
    concesionario.agregar_auto(auto1);
    concesionario.agregar_auto(auto2);
    concesionario.agregar_auto(auto3);

    // Crear un auto para buscar
    let auto_buscado = Auto::new(
        String::from("BMW"),
        String::from("X5"),
        2020,
        50000.0,
        Color::NEGRO,
    );

    // Buscar el auto
    match concesionario.buscar_auto(&auto_buscado) {
        Some(auto_encontrado) => println!("Auto encontrado: {:#?} ", auto_encontrado),
        None => println!("Auto no encontrado"),
    }
    // eliminar y buscar el auto
    concesionario.eliminar_auto(&auto_buscado);
    match concesionario.buscar_auto(&auto_buscado) {
        Some(auto_encontrado) => println!("Auto encontrado: {:#?} ", auto_encontrado),
        None => println!("Auto no encontrado"),
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

        assert_eq!(concesionario.agregar_auto(auto1), true);
        assert_eq!(concesionario.agregar_auto(auto2), true);
        assert_eq!(concesionario.agregar_auto(auto3), false);
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

        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2);

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

        concesionario.agregar_auto(auto1.clone());
        concesionario.agregar_auto(auto2);

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
