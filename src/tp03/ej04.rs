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

#[derive(Debug)]
enum TriTipo{
    ESCALENO,// Escaleno: Todos los lados son desiguales
    ISOSCELES,// Isósceles: Dos lados son iguales y uno diferente
    EQUILATERO // Equilátero: Todos los lados son iguales
}




impl Triangulo {
    fn new (lado_1: f64, lado_2: f64, lado_3: f64) -> Triangulo{
        Triangulo{lado_1, lado_2, lado_3}
    }

    fn determinar_tipo (&self) -> TriTipo{
        if self.lado_1 != self.lado_2 && self.lado_1 != self.lado_3 && self.lado_3 != self.lado_2 {
            TriTipo::ESCALENO
        } else if self.lado_1 == self.lado_2 && self.lado_1 == self.lado_3 && self.lado_3 == self.lado_2 {
            TriTipo::EQUILATERO
        } else {
            TriTipo::ISOSCELES
        }
    }

    fn calcular_area (&self) -> f64{
        // Fórmula de Herón	A = √[ p × (p - L1) × (p - L2) × (p - L3) ]

        // let mut b: f64;
        // let mut h: f64;
        // match &self.determinar_tipo() {
        //     TriTipo::ESCALENO => {

        //     },
        //     TriTipo::EQUILATERO => {

        //     },
        //     TriTipo::ISOSCELES => {

        //     }
        // }

        ((self.calcular_perimetro() - self.lado_1) * (self.calcular_perimetro() - self.lado_2) * (self.calcular_perimetro() - self.lado_3) ).sqrt()
    }
    
    fn calcular_perimetro (&self) -> f64{
        self.lado_1 + self.lado_2 + self.lado_3
    }
}

pub fn run(){
    let lado_1: f64 = 22.8;
    let lado_2: f64 = 5.64;
    let lado_3: f64 = 22.8;
    let t1 = Triangulo::new(lado_1, lado_2, lado_3);

    println!("{:?}", t1 );
    println!("{:?}", t1.determinar_tipo() );
    println!("{:?}", t1.calcular_perimetro() );
    println!("{:?}", t1.calcular_area() );
    
}



