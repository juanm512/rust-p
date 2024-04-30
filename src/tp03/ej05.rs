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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculacion_impuestos() {
        let producto = Producto {
            id: String::from("ID123"),
            nombre: String::from("Producto A"),
            precio_bruto: 100.0,
        };

        let impuestos = producto.calcular_impuestos(20.0); // Asumiendo un 20% de impuesto
        assert!((impuestos - 20.0).abs() < f32::EPSILON, "El cálculo del impuesto no es correcto");
    }

    #[test]
    fn test_aplicacion_descuento() {
        let producto = Producto {
            id: String::from("ID124"),
            nombre: String::from("Producto B"),
            precio_bruto: 80.0,
        };

        let descuento = producto.aplicar_descuento(30.0); // Asumiendo un 30% de descuento
        assert!((24.0 - descuento).abs() < f32::EPSILON, "El cálculo del descuento no es correcto");
    }

    #[test]
    fn test_calcular_precio_total_con_impuesto_y_descuento() {
        let producto = Producto {
            id: String::from("ID125"),
            nombre: String::from("Producto C"),
            precio_bruto: 60.0,
        };

        // Prueba con ambos parámetros pasados
        let precio_total = producto.calcular_precio_total(Some(10.0), Some(25.0));
        assert!((precio_total - 51.0).abs() < f32::EPSILON, "El cálculo del precio total no es correcto con impuestos y descuento");

        // Prueba sin parámetros (asumiendo un 10% de impuesto y no aplicar descuento)
        let precio_total_sin_parámetros = producto.calcular_precio_total(Some(10.0), None);
        assert!((60.0 + (60.0 * 10.0 / 100.0)) - precio_total_sin_parámetros.abs() < f32::EPSILON, "El cálculo del precio total no es correcto sin parámetros");
    }
}
