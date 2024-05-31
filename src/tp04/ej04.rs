// 4 -Se requiere implementar un sistema de ventas de productos. De cada producto se
// conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
// descuentos aplicables dependiendo de la categoría. Además, se debe registrar al vendedor
// que realizo la venta y al cliente. De ellos se conoce nombre, apellido, direccion, dni y del
// vendedor nro de legajo, antigüedad y salario. Los clientes pueden tener un beneficio de
// descuento si tienen suscripcion al newsletter, de ser así se tiene el correo electronico del
// mismo.

// El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
// Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
// bancaria y efectivo.
// Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
// siguientes acciones:
//      ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
//          productos.
//      ➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
//          calcularlo tenga en cuenta que pueden haber determinados productos de alguna
//          categoría donde debería aplicarse un descuento. Tanto la categoría como el
//          porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
//          sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
//          aplicar un porcentaje de descuento general si el cliente tiene suscripcion al
//          newsletter.
//      ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
//          permita visualizar las ventas totales por categoría de producto y otro por vendedor.

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_

use crate::tp03::ej03::Fecha;
use std::{collections::HashMap, hash::Hash};
const DESCUENTO_NEWSLETTER: f32 = 0.1;

#[derive(Debug, Clone, PartialEq)]
enum MediosDePago {
    Efectivo,
    TarjetaDeDebito(TarjetaDeDebitoData),
    TarjetaDeCredito(TarjetaDeCreditoData),
    TransferenciaBancaria(TransferenciaBancariaData),
}

#[derive(Debug, Clone, PartialEq)]
struct TarjetaDeDebitoData {
    card_number: String,
    card_holder_name: String,
    expiration_date: String,
    cvv: String,
}

#[derive(Debug, Clone, PartialEq)]
struct TarjetaDeCreditoData {
    card_number: String,
    card_holder_name: String,
    expiration_date: String,
    cvv: String,
}

#[derive(Debug, Clone, PartialEq)]
struct TransferenciaBancariaData {
    bank_name: String,
    account_number: String,
    routing_number: String,
}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct DatosPersona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}
trait Persona {
    fn get_nombre(&self, datos: &DatosPersona) -> String {
        datos.nombre.clone()
    }
    fn get_apellido(&self, datos: &DatosPersona) -> String {
        datos.apellido.clone()
    }
    fn get_direccion(&self, datos: &DatosPersona) -> String {
        datos.direccion.clone()
    }
    fn get_dni(&self, datos: &DatosPersona) -> u32 {
        datos.dni
    }
}
#[derive(Debug, Clone)]
struct Vendedor {
    datos_persona: DatosPersona,
    legajo: u32,
    antigüedad: u8,
    salario: f32,
}
impl Eq for Vendedor {}
impl PartialEq for Vendedor {
    fn eq(&self, other: &Self) -> bool {
        self.datos_persona == other.datos_persona
            && self.legajo == other.legajo
            && self.antigüedad == other.antigüedad
            && self.salario == other.salario
    }
}
impl Hash for Vendedor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.datos_persona.hash(state);
        self.legajo.hash(state);
        self.antigüedad.hash(state);
    }
}

impl Vendedor {
    fn new(
        legajo: u32,
        antigüedad: u8,
        salario: f32,
        nombre: String,
        apellido: String,
        direccion: String,
        dni: u32,
    ) -> Vendedor {
        Vendedor {
            datos_persona: DatosPersona {
                nombre,
                apellido,
                direccion,
                dni,
            },
            legajo,
            antigüedad,
            salario,
        }
    }
}
impl Persona for Vendedor {}

#[derive(Debug)]
struct Newsletter {
    email: String, //mas campos se podrian agregar si fuera necesario
}
#[derive(Debug)]
struct Cliente {
    datos_persona: DatosPersona,
    newsletter: Option<Newsletter>,
}

impl Cliente {
    fn new(
        newsletter: Option<Newsletter>,
        nombre: String,
        apellido: String,
        direccion: String,
        dni: u32,
    ) -> Cliente {
        Cliente {
            datos_persona: DatosPersona {
                nombre,
                apellido,
                direccion,
                dni,
            },
            newsletter,
        }
    }

    fn aplicar_descuento(&self, monto: f32) -> f32 {
        match &self.newsletter {
            Some(_) => monto * (1.0 + DESCUENTO_NEWSLETTER),
            None => monto,
        }
    }
}
impl Persona for Cliente {}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Categorias {
    A,
    B,
    C,
    D,
}

#[derive(Debug)]
struct Producto {
    nombre: String,
    categoria: Categorias,
    precio_base: f32,
}
impl Producto {
    fn get_categoria(&self) -> Categorias {
        self.categoria.clone()
    }
    fn get_precio_base(&self) -> f32 {
        self.precio_base
    }
}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_
#[derive(Debug)]
struct Venta<'a> {
    fecha: Fecha,
    cliente: &'a Cliente,
    vendedor: &'a Vendedor,
    medio_de_pago: MediosDePago,
    productos: Vec<&'a Producto>,
}

impl<'a> Venta<'a> {
    fn new(
        fecha: Fecha,
        cliente: &'a Cliente,
        vendedor: &'a Vendedor,
        medio_de_pago: MediosDePago,
        productos: Vec<&'a Producto>,
    ) -> Venta<'a> {
        Venta {
            fecha,
            cliente,
            vendedor,
            medio_de_pago,
            productos,
        }
    }

    fn calcular_precio_final(&self, sistema: &Sistema) -> f32 {
        let mut total = 0.0;
        for producto in self.productos.iter() {
            let desc = sistema.get_descuento_categoria(producto.get_categoria());
            total += producto.get_precio_base() * (1.0 + desc);
        }
        self.cliente.aplicar_descuento(total)
    }
}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_

#[derive(Debug)]
struct Sistema<'a> {
    ventas: Vec<Venta<'a>>,
}

impl<'a> Sistema<'a> {
    fn new() -> Sistema<'a> {
        Sistema { ventas: Vec::new() }
    }

    fn cargar_venta(&mut self, venta: Venta<'a>) {
        self.ventas.push(venta);
    }

    fn get_descuento_categoria(&self, cat: Categorias) -> f32 {
        match cat {
            Categorias::A => 0.02,
            Categorias::B => 0.2,
            Categorias::C => 0.0,
            Categorias::D => 0.1,
        }
    }

    fn generar_reporte_por_categoria(&self) -> HashMap<Categorias, u32> {
        let mut ventas_por_categoria: HashMap<Categorias, u32> = HashMap::new();
        for venta in self.ventas.iter() {
            for producto in venta.productos.iter() {
                let categoria = producto.get_categoria();
                *ventas_por_categoria.entry(categoria).or_insert(0) += 1;
            }
        }
        ventas_por_categoria
    }

    fn generar_reporte_por_vendedor(&self) -> HashMap<Vendedor, u32> {
        let mut ventas_por_vendedor: HashMap<Vendedor, u32> = HashMap::new();
        for venta in self.ventas.iter() {
            let vendedor = venta.vendedor.clone();
            *ventas_por_vendedor.entry(vendedor).or_insert(0) += 1;
        }

        ventas_por_vendedor
    }
}
// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_
#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::Rng;

    fn create_fecha() -> Fecha {
        let mut rng = rand::thread_rng();
        let year = rng.gen_range(2022..=2024);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28); // Simplificación para evitar meses con menos de 31 días
        Fecha::new(year, month, day) // Suposición de que `Fecha::new(year, month, day)` existe
    }

    fn create_cliente(con_newsletter: bool) -> Cliente {
        Cliente::new(
            if con_newsletter {
                Some(Newsletter {
                    email: "test@example.com".to_string(),
                })
            } else {
                None
            },
            "Juan".to_string(),
            "Perez".to_string(),
            "123 Fake Street".to_string(),
            12345678,
        )
    }

    fn create_vendedor(index: u8) -> Vendedor {
        match index {
            1 => Vendedor::new(
                1,
                5,
                1000.0,
                "Juan".to_string(),
                "Perez".to_string(),
                "direccion".to_string(),
                123,
            ),
            2 => Vendedor::new(
                2,
                10,
                2000.0,
                "Maria".to_string(),
                "Gomez".to_string(),
                "direccion2".to_string(),
                456,
            ),
            3 => Vendedor::new(
                3,
                15,
                3000.0,
                "Pedro".to_string(),
                "Lopez".to_string(),
                "direccion3".to_string(),
                789,
            ),
            _ => panic!("Vendedor index out of range"),
        }
    }

    fn create_producto(index: u8) -> Producto {
        match index {
            1 => Producto {
                nombre: "Producto 1".to_string(),
                categoria: Categorias::A,
                precio_base: 100.0,
            },
            2 => Producto {
                nombre: "Producto 2".to_string(),
                categoria: Categorias::B,
                precio_base: 200.0,
            },
            3 => Producto {
                nombre: "Producto 3".to_string(),
                categoria: Categorias::A,
                precio_base: 50.0,
            },
            4 => Producto {
                nombre: "Producto 4".to_string(),
                categoria: Categorias::C,
                precio_base: 300.0,
            },
            5 => Producto {
                nombre: "Producto 5".to_string(),
                categoria: Categorias::B,
                precio_base: 250.0,
            },
            _ => panic!("Producto index out of range"),
        }
    }

    #[test]
    fn test_persona_obtener_datos() {
        let nombre = "Juan".to_string();
        let apellido = "Alvarez".to_string();
        let direccion = "123 Fake Street".to_string();
        let dni = 37856132;

        let cliente = create_cliente(true);

        assert_eq!(nombre, cliente.get_nombre(&cliente.datos_persona));
        assert_ne!(apellido, cliente.get_apellido(&cliente.datos_persona));
        assert_eq!(direccion, cliente.get_direccion(&cliente.datos_persona));
        assert_ne!(dni, cliente.get_dni(&cliente.datos_persona));
    }

    #[test]
    fn test_cliente_aplicar_descuento_con_newsletter() {
        let cliente = create_cliente(true);
        let monto = 100.0;
        let monto_con_descuento = cliente.aplicar_descuento(monto);
        assert_eq!(monto_con_descuento, monto * (1.0 + DESCUENTO_NEWSLETTER));
    }

    #[test]
    fn test_cliente_aplicar_descuento_sin_newsletter() {
        let cliente = create_cliente(false);
        let monto = 100.0;
        let monto_sin_descuento = cliente.aplicar_descuento(monto);
        assert_eq!(monto_sin_descuento, monto);
    }

    #[test]
    fn test_calcular_precio_final() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto1 = create_producto(1);
        let producto2 = create_producto(2);
        let productos = vec![&producto1, &producto2];
        let venta = Venta::new(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
        );

        let sistema = Sistema::new();
        let precio_final = venta.calcular_precio_final(&sistema);
        let precio_esperado = (100.0 * 1.02 + 200.0 * 1.2) * (1.0 + DESCUENTO_NEWSLETTER);
        assert_eq!(precio_final, precio_esperado);
    }

    #[test]
    fn test_sistema_cargar_venta() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto1 = create_producto(1);
        let producto2 = create_producto(2);
        let productos = vec![&producto1, &producto2];
        let venta = Venta::new(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
        );
        let mut sistema = Sistema::new();
        sistema.cargar_venta(venta);

        assert_eq!(sistema.ventas.len(), 1);
    }

    #[test]
    fn test_sistema_generar_reporte_por_categoria() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto_a = create_producto(1);
        let producto_b = create_producto(2);
        let productos = vec![&producto_a, &producto_b];

        let venta = Venta::new(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
        );

        let mut sistema = Sistema::new();
        sistema.cargar_venta(venta);

        let reporte = sistema.generar_reporte_por_categoria();

        assert_eq!(*reporte.get(&Categorias::A).unwrap(), 1);
        assert_eq!(*reporte.get(&Categorias::B).unwrap(), 1);
    }

    #[test]
    fn test_sistema_generar_reporte_por_vendedor() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto = create_producto(1);
        let productos = vec![&producto];

        let venta = Venta::new(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
        );

        let mut sistema = Sistema::new();
        sistema.cargar_venta(venta);

        let reporte = sistema.generar_reporte_por_vendedor();

        assert_eq!(*reporte.get(&vendedor).unwrap(), 1);
    }

    #[test]
    fn test_varias_ventas_diferentes_vendedores() {
        let cliente = create_cliente(true);
        let fecha = create_fecha();
        let producto1 = create_producto(1);
        let producto2 = create_producto(2);
        let producto3 = create_producto(3);

        let vendedor1 = create_vendedor(1);
        let vendedor2 = create_vendedor(2);

        let venta1 = Venta::new(
            fecha.clone(),
            &cliente,
            &vendedor1,
            MediosDePago::Efectivo,
            vec![&producto1],
        );
        let venta2 = Venta::new(
            fecha.clone(),
            &cliente,
            &vendedor2,
            MediosDePago::Efectivo,
            vec![&producto2, &producto3],
        );

        let mut sistema = Sistema::new();
        sistema.cargar_venta(venta1);
        sistema.cargar_venta(venta2);

        let reporte_vendedores = sistema.generar_reporte_por_vendedor();
        assert_eq!(*reporte_vendedores.get(&vendedor1).unwrap(), 1);
        assert_eq!(*reporte_vendedores.get(&vendedor2).unwrap(), 1);
    }
}
