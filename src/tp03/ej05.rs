// 5- Escribir un programa que defina una estructura Producto que tenga campos para el
// nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
// siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
// ➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
// el precio bruto
// ➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
// descuento sobre el precio bruto
// ➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
// precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
// parámetros son opcionales.


#[derive(Debug)]
struct Producto{
    id: String,
    nombre: String,
    precio_bruto: f32,
}

impl Producto{
    pub fn new(id: String, nombre: String, precio_bruto: f32) -> Producto{
        Producto{
            id, nombre, precio_bruto
        }
    }
    pub fn calcular_impuestos(&self, porcentaje_de_impuestos: f32) -> f32 {
        (porcentaje_de_impuestos / 100.0) * self.precio_bruto
    }
    pub fn aplicar_descuento(&self, porcentaje_de_descuento: f32) -> f32 {
        (porcentaje_de_descuento / 100.0) * self.precio_bruto
    }
    pub fn calcular_precio_total(&self, porcentaje_de_impuestos: Option<f32>, porcentaje_descuento: Option<f32>) -> f32 {
        let mut precio_total = self.precio_bruto;
        
        if let Some(data) = porcentaje_de_impuestos {
            precio_total += self.calcular_impuestos(data);
        }

        if let Some(data2) = porcentaje_descuento {
            precio_total -= self.aplicar_descuento(data2);
        }

        precio_total
    }
}


// pub fn run(){
//     let id = String::from("");    
//     let nombre = String::from("");    
//     let precio: f32= 15.5;

//     let p = Producto::new(id, nombre, precio);

//     println!("{:#?}", p);
//     println!("{:#?}", p.calcular_impuestos(10.0) );
//     println!("{:#?}", p.aplicar_descuento(20.0) );
//     println!("{:#?}", p.calcular_precio_total( Some(10.0), Some(30.0) ) );
//     println!("{:#?}", p.calcular_precio_total( Some(10.0), None ) );
// }


// HACER TESTS