// 4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
// longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
// ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
// isósceles o escaleno.
// ➢ calcular_area: calcular el área y la retorna.
// ➢ calcular_perimetro: calcula el perímetro y lo retorna.

#[derive(Debug)]
struct Triangulo {
    lado_1: f64,
    lado_2: f64,
    lado_3: f64,
}
#[derive(PartialEq, Debug)]
enum TriTipo {
    ESCALENO,   // Escaleno: Todos los lados son desiguales
    ISOSCELES,  // Isósceles: Dos lados son iguales y uno diferente
    EQUILATERO, // Equilátero: Todos los lados son iguales
}

impl Triangulo {
    pub fn new(lado_1: f64, lado_2: f64, lado_3: f64) -> Triangulo {
        Triangulo {
            lado_1,
            lado_2,
            lado_3,
        }
    }

    pub fn determinar_tipo(&self) -> TriTipo {
        if self.lado_1 != self.lado_2 && self.lado_1 != self.lado_3 && self.lado_3 != self.lado_2 {
            TriTipo::ESCALENO
        } else if self.lado_1 == self.lado_2
            && self.lado_1 == self.lado_3
            && self.lado_3 == self.lado_2
        {
            TriTipo::EQUILATERO
        } else {
            TriTipo::ISOSCELES
        }
    }

    pub fn calcular_area(&self) -> f64 {
        // Fórmula de Herón A = √[ p × (p - L1) × (p - L2) × (p - L3) ]
        (self.calcular_perimetro()
            * 0.5
            * (self.calcular_perimetro() * 0.5 - self.lado_1)
            * (self.calcular_perimetro() * 0.5 - self.lado_2)
            * (self.calcular_perimetro() * 0.5 - self.lado_3))
            .sqrt()
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado_1 + self.lado_2 + self.lado_3
    }
}

pub fn run() {}

#[test]
fn test_determinar_tipo_isosceles() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 5.64;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(TriTipo::ISOSCELES, t1.determinar_tipo());
}

#[test]
fn test_determinar_tipo_escaleno() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 5.64;
    let lado_3: f64 = 9.1;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(TriTipo::ESCALENO, t1.determinar_tipo());
}

#[test]
fn test_determinar_tipo_equilatero() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 22.8;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(TriTipo::EQUILATERO, t1.determinar_tipo());
}

#[test]
fn test_calcular_perimetro_equilatero() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 22.8;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(lado_1 + lado_2 + lado_3, t1.calcular_perimetro());
}
#[test]
fn test_calcular_perimetro_escaleno() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 12.8;
    let lado_3: f64 = 34.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(lado_1 + lado_2 + lado_3, t1.calcular_perimetro());
}
#[test]
fn test_calcular_perimetro_isosceles() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 2.8;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(lado_1 + lado_2 + lado_3, t1.calcular_perimetro());
}

fn correctans(num: f64) -> f64 {
    (num * 100000.0).round() / 100000.0
}

#[test]
fn test_calcular_area_equilatero() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 22.8;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(225.09732, correctans(t1.calcular_area()));
    assert_ne!(200.09732, t1.calcular_area());
}
#[test]
fn test_calcular_area_escaleno() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 12.8;
    let lado_3: f64 = 34.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(62.53688, correctans(t1.calcular_area()));
    assert_ne!(32.53236, t1.calcular_area());
}
#[test]
fn test_calcular_area_isosceles() {
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 2.8;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);
    assert_eq!(31.85977, correctans(t1.calcular_area()));
    assert_ne!(321.85976, t1.calcular_area());
}
