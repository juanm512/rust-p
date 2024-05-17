// juan manuel, vila, 20572/7, juanm512

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
struct DatosAutoReporte {
    modelo: String,
    año: u32,
    precio: f32,
    color: Color,
}

#[derive(Debug)]
struct Reporte {
    autos: Vec<DatosAutoReporte>,
    cantidad_autos_marca: u32,
}

#[derive(Debug)]
struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f32,
    color: Color,
}

impl Auto {
    pub fn new(marca: String, modelo: String, año: u32, precio_bruto: f32, color: Color) -> Auto {
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

#[derive(Debug)]
struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    max_cap: u32,
    autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, max_cap: u32) -> ConcesionarioAuto {
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
                    break;
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

    pub fn reporte_de_autos_por_marca(&self, marca: String) -> Reporte {
        let mut reporte_ret = Reporte {
            autos: Vec::new(),
            cantidad_autos_marca: 0,
        };

        for auto in &self.autos {
            if auto.marca == marca {
                reporte_ret.autos.push(DatosAutoReporte {
                    modelo: auto.modelo.clone(),
                    año: auto.año.clone(),
                    precio: auto.calcular_precio(),
                    color: auto.color.clone(),
                });
            }
        }

        reporte_ret.cantidad_autos_marca = reporte_ret.autos.len() as u32;
        reporte_ret
    }
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
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
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
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
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

#[test]
fn test_concesionario_buscar_auto() {
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
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

#[test]
fn test_concesionario_reporte_autos_por_marca() {
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
    let auto4 = Auto::new(
        String::from("Toyota"),
        String::from("Etios"),
        2015,
        12000.0,
        Color::NEGRO,
    );

    // Crear un concesionario
    let mut concesionario = ConcesionarioAuto::new(
        String::from("Concesionario XYZ"),
        String::from("Calle Principal"),
        6,
    );

    // Agregar autos al concesionario
    concesionario.agregar_auto(auto1.clone());
    concesionario.agregar_auto(auto2.clone());
    concesionario.agregar_auto(auto3.clone());
    concesionario.agregar_auto(auto4.clone());

    let reporte = concesionario.reporte_de_autos_por_marca("Toyota".to_string());
    assert_eq!(reporte.cantidad_autos_marca, 2);
    assert_eq!(reporte.autos[0].modelo, String::from("Corolla"));
    assert_eq!(reporte.autos[1].modelo, String::from("Etios"));
}
#[test]
fn test_concesionario_reporte_autos_por_marca_sin_autos() {
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
    let auto4 = Auto::new(
        String::from("Toyota"),
        String::from("Etios"),
        2015,
        12000.0,
        Color::NEGRO,
    );

    // Crear un concesionario
    let mut concesionario = ConcesionarioAuto::new(
        String::from("Concesionario XYZ"),
        String::from("Calle Principal"),
        6,
    );

    // Agregar autos al concesionario
    concesionario.agregar_auto(auto1.clone());
    concesionario.agregar_auto(auto2.clone());
    concesionario.agregar_auto(auto3.clone());
    concesionario.agregar_auto(auto4.clone());

    let reporte = concesionario.reporte_de_autos_por_marca("Porsche".to_string());
    assert_eq!(reporte.cantidad_autos_marca, 0);
    assert!(reporte.autos.is_empty());
}
