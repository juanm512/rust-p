// 2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
// longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
// retorna.
// ➢ calcular_area: calcular el área y la retorna.
// ➢ calcular_perimetro: calcula el perímetro y lo retorna.
// ➢ es_cuadrado: retorna true si es cuadrado, false caso contrario


#[derive(Debug)]
struct Rectangulo {
    longitud: f64,
    ancho: f64
}

impl Rectangulo {
    fn new (longitud: f64, ancho: f64) -> Rectangulo{
        Rectangulo{longitud, ancho}
    }

    fn calcular_area (&self) -> f64{
        self.ancho * self.longitud
    }

    fn calcular_perimetro (&self) -> f64{
        (self.ancho * 2.0) + (self.longitud * 2.0)
    }

    fn es_cuadrado (&self) -> bool{
        self.ancho == self.longitud
    }
}

pub fn run(){
    let a = 3.1;
    let l = 1.765;
    let r = Rectangulo::new(l,a);

    println!("{:?}", r );
    println!("Area: {:?}", r.calcular_area() );
    println!("Perimetro: {:?}", r.calcular_perimetro() );
    println!("Es cuadrado: {:?}", r.es_cuadrado() );
}

