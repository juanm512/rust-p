// juan manuel, vila, 20572/7, juanm512

use std::{
    collections::{HashMap, LinkedList},
    hash::Hash,
};
const DESCUENTO_NEWSLETTER: f32 = 0.1;

#[derive(Debug)]
pub(crate) struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
        Fecha { dia, mes, anio }
    }
    pub fn equals(&self, other: &Self) -> bool {
        self.dia == other.dia && self.mes == other.mes && self.anio == other.anio
    }
    pub fn clone(&self) -> Self {
        Self::new(self.dia.clone(), self.mes.clone(), self.anio.clone())
    }

    fn calcular_dias_mes(&self) -> u32 {
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => return 31,
            4 | 6 | 9 | 11 => return 30,
            2 => {
                if self.es_bisiesto() {
                    return 29;
                }
                return 28;
            }
            _ => 30,
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes <= 12 && self.dia <= self.calcular_dias_mes() {
            return true;
        }
        return false;
    }

    pub fn es_bisiesto(&self) -> bool {
        self.anio % 4 == 0 && self.anio % 100 != 0
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            if self.dia + dias > self.calcular_dias_mes() {
                dias = dias - (self.calcular_dias_mes() - self.dia) - 1;
                self.dia = 1;
                if self.mes == 12 {
                    self.mes = 1;
                    self.anio += 1;
                } else {
                    self.mes += 1;
                }
            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }
    pub fn restar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            if (self.dia as i32 - dias as i32) < 1 {
                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                dias = dias - (self.dia);
                self.dia = self.calcular_dias_mes();
            } else {
                self.dia -= dias;
                dias = 0;
            }
        }
    }
    pub fn es_mayor(&self, fecha: &Fecha) -> bool {
        if self.anio > fecha.anio {
            return true;
        } else if self.anio == fecha.anio {
            if self.mes > fecha.mes {
                return true;
            } else if self.mes == fecha.mes {
                if self.dia > fecha.dia {
                    return true;
                }
                return self.dia == fecha.dia;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

// ================================
// FIN DE FECHA ===================
// ================================

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
// Fin Persona/DatosPersona - Inicio Cliente
#[derive(Debug, Clone)]
struct Newsletter {
    email: String, //mas campos se podrian agregar si fuera necesario
}
#[derive(Debug, Clone)]
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
// Fin Cliente - Inicio Vendedor
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
    pub fn new(
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
    stock: u32,
}
impl Producto {
    fn new(nombre: String, categoria: Categorias, precio_base: f32, stock: u32) -> Producto {
        Producto {
            nombre,
            categoria,
            precio_base,
            stock,
        }
    }
    fn get_categoria(&self) -> Categorias {
        self.categoria.clone()
    }
    fn get_precio_base(&self) -> f32 {
        self.precio_base
    }
    fn get_stock(&self) -> u32 {
        self.stock
    }
    fn reducir_stock(&mut self, cantidad: u32) {
        self.stock -= cantidad;
    }
}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_
#[derive(Debug)]
struct Venta<'a> {
    fecha: Fecha,
    cliente: &'a Cliente,
    vendedor: &'a Vendedor,
    medio_de_pago: MediosDePago,
    // productos: Vec<&'a Producto>,
    cantidades_productos: HashMap<String, u32>,
}

impl<'a> Venta<'a> {
    fn new(
        fecha: Fecha,
        cliente: &'a Cliente,
        vendedor: &'a Vendedor,
        medio_de_pago: MediosDePago,
        cantidades_productos: HashMap<String, u32>,
    ) -> Venta<'a> {
        Venta {
            fecha,
            cliente,
            vendedor,
            medio_de_pago,
            cantidades_productos,
        }
    }

    fn calcular_precio_final(&self, sistema: &Sistema) -> f32 {
        let mut total = 0.0;
        for producto in sistema.productos.iter() {
            let desc = sistema.get_descuento_categoria(producto.1.get_categoria());
            total += producto.1.get_precio_base()
                * (1.0 + desc)
                * (*self.cantidades_productos.get(&producto.1.nombre).unwrap() as f32);
        }
        self.cliente.aplicar_descuento(total)
    }
}

// _-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-__-_-_

#[derive(Debug)]
struct Sistema<'a> {
    // productos: Vec<Producto>,
    productos: HashMap<String, Producto>,
    clientes: Vec<Cliente>,
    vendedores: Vec<Vendedor>,
    ventas: Vec<Venta<'a>>,
}

impl<'a> Sistema<'a> {
    fn new() -> Sistema<'a> {
        Sistema {
            productos: HashMap::new(),
            clientes: Vec::new(),
            vendedores: Vec::new(),
            ventas: Vec::new(),
        }
    }

    fn cargar_venta(
        &mut self,
        fecha: Fecha,
        cliente: &'a Cliente,
        vendedor: &'a Vendedor,
        medio_de_pago: MediosDePago,
        cantidades_productos: HashMap<String, u32>,
    ) -> bool {
        if self
            .productos
            .iter()
            .filter(|p| cantidades_productos.iter().any(|cp| *cp.0 == p.1.nombre))
            .zip(cantidades_productos.values())
            .any(|par_prod_cant| par_prod_cant.0 .1.get_stock() < *par_prod_cant.1)
        {
            return false; // Stock insuficiente
        }

        // Actualizar stock
        for (producto, &cantidad) in self.productos.iter_mut().zip(cantidades_productos.values()) {
            producto.1.reducir_stock(cantidad);
        }

        let venta = Venta::new(
            fecha,
            cliente,
            vendedor,
            medio_de_pago,
            cantidades_productos,
        );

        self.ventas.push(venta);
        true
    }

    fn agregar_producto(&mut self, producto: Producto) {
        self.productos.insert(producto.nombre.clone(), producto);
    }

    fn agregar_cliente(&mut self, cliente: Cliente) {
        self.clientes.push(cliente);
    }

    fn agregar_vendedor(&mut self, vendedor: Vendedor) {
        self.vendedores.push(vendedor);
    }

    fn get_descuento_categoria(&self, cat: Categorias) -> f32 {
        match cat {
            Categorias::A => 0.02,
            Categorias::B => 0.2,
            Categorias::C => 0.0,
            Categorias::D => 0.1,
        }
    }

    fn generar_reporte_productos_sin_stock(&self, cantidad_limite: u32) -> LinkedList<&Producto> {
        let mut productos_sin_stock: LinkedList<&Producto> = LinkedList::new();
        self.productos.iter().for_each(|p| {
            if p.1.get_stock() < cantidad_limite {
                productos_sin_stock.push_back(&p.1);
            }
        });
        productos_sin_stock
    }

    fn generar_reporte_por_categoria(&self) -> HashMap<Categorias, u32> {
        let mut ventas_por_categoria: HashMap<Categorias, u32> = HashMap::new();
        for venta in self.ventas.iter() {
            for producto in self.productos.iter().filter(|p| {
                venta
                    .cantidades_productos
                    .iter()
                    .any(|cp| *cp.0 == p.1.nombre)
            }) {
                let categoria = producto.1.get_categoria();
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

pub fn run() {
    // Crear productos
    let producto1 = Producto {
        nombre: String::from("Producto 1"),
        categoria: Categorias::A,
        precio_base: 100.0,
        stock: 10,
    };

    let producto2 = Producto {
        nombre: String::from("Producto 2"),
        categoria: Categorias::B,
        precio_base: 150.0,
        stock: 5,
    };

    // Crear cliente y vendedor
    let cliente = Cliente::new(
        Some(Newsletter {
            email: String::from("cliente@example.com"),
        }),
        String::from("Nombre"),
        String::from("Apellido"),
        String::from("Dirección"),
        12345678,
    );
    let vendedor = Vendedor::new(
        12345678,
        1,
        50000.0,
        String::from("Vendedor"),
        String::from("Apellido"),
        String::from("Dirección"),
        12345678,
    );

    // Intentar crear una venta

    let mut sistema = Sistema::new();

    sistema.agregar_cliente(cliente.clone());
    sistema.agregar_vendedor(vendedor.clone());
    sistema.agregar_producto(producto1);
    sistema.agregar_producto(producto2);

    println!();
    println!();
    println!("sistema: {:#?}", sistema);
    println!();
    println!();

    let cantidades_productos = HashMap::from([
        (String::from("Producto 1"), 3),
        (String::from("Producto 2"), 2),
    ]);
    let venta = sistema.cargar_venta(
        Fecha::new(1, 1, 2023),
        &cliente,
        &vendedor,
        MediosDePago::Efectivo,
        cantidades_productos,
    );
    match venta {
        true => {
            println!("Venta creada con éxito: {:#?}", venta);
        }
        false => println!("Stock insuficiente para realizar la venta."),
    }
    println!();
    println!();
    println!("sistema: {:#?}", sistema);
    println!();
    println!();

    // Generar reporte de productos bajo stock
    let productos_bajo_stock = sistema.generar_reporte_productos_sin_stock(6);
    println!("Productos con stock bajo: {:?}", productos_bajo_stock);
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
                nombre: "Coca-Cola".to_string(),
                categoria: Categorias::A,
                precio_base: 100.0,
                stock: 12,
            },
            2 => Producto {
                nombre: "Guaymallen Triple Blanco".to_string(),
                categoria: Categorias::B,
                precio_base: 200.0,
                stock: 2,
            },
            3 => Producto {
                nombre: "Fanta".to_string(),
                categoria: Categorias::A,
                precio_base: 50.0,
                stock: 47,
            },
            4 => Producto {
                nombre: "Mantela La Serenisima".to_string(),
                categoria: Categorias::C,
                precio_base: 300.0,
                stock: 8,
            },
            5 => Producto {
                nombre: "Aguila MiniTorta".to_string(),
                categoria: Categorias::B,
                precio_base: 250.0,
                stock: 100,
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
        let cantidades: HashMap<String, u32> =
            HashMap::from([(producto1.nombre.clone(), 2), (producto2.nombre.clone(), 1)]);
        let productos = vec![&producto1, &producto2];
        let venta = Venta::new(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
            cantidades,
        );

        let sistema = Sistema::new();
        let precio_final = venta.calcular_precio_final(&sistema);
        let precio_esperado = (100.0 * 1.02 + 200.0 * 1.2) * (1.0 + DESCUENTO_NEWSLETTER);
        assert_eq!(precio_final, precio_esperado);
    }

    #[test]
    fn test_sistema_cargar_producto() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto1 = create_producto(1);
        let producto2 = create_producto(2);
        let cantidades: HashMap<String, u32> =
            HashMap::from([(producto1.nombre.clone(), 2), (producto2.nombre.clone(), 1)]);
        let productos = vec![&producto1, &producto2];

        let mut sistema = Sistema::new();
        sistema.cargar_venta(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
            cantidades,
        );

        assert_eq!(sistema.ventas.len(), 1);
    }

    #[test]
    fn test_sistema_cargar_venta() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto1 = create_producto(1);
        let producto2 = create_producto(2);
        let cantidades: HashMap<String, u32> =
            HashMap::from([(producto1.nombre.clone(), 2), (producto2.nombre.clone(), 1)]);
        let productos = vec![&producto1, &producto2];
        let mut sistema = Sistema::new();
        sistema.cargar_venta(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
            cantidades,
        );

        assert_eq!(sistema.ventas.len(), 1);
    }

    #[test]
    fn test_sistema_generar_reporte_por_categoria() {
        let cliente = create_cliente(true);
        let vendedor = create_vendedor(1);
        let fecha = create_fecha();
        let producto_a = create_producto(1);
        let producto_b = create_producto(2);
        let cantidades: HashMap<String, u32> = HashMap::from([
            (producto_a.nombre.clone(), 2),
            (producto_b.nombre.clone(), 1),
        ]);
        let productos = vec![&producto_a, &producto_b];

        let mut sistema = Sistema::new();
        sistema.cargar_venta(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            productos,
            cantidades,
        );

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
        let cantidades: HashMap<String, u32> = HashMap::from([(producto.nombre.clone(), 2)]);

        let mut sistema = Sistema::new();
        let prod = sistema.cargar_producto(&producto);
        sistema.cargar_venta(
            fecha,
            &cliente,
            &vendedor,
            MediosDePago::Efectivo,
            vec![&prod],
            cantidades,
        );

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

        let cantidades1: HashMap<String, u32> = HashMap::from([(producto1.nombre.clone(), 2)]);
        let cantidades2: HashMap<String, u32> =
            HashMap::from([(producto2.nombre.clone(), 2), (producto3.nombre.clone(), 5)]);

        let mut sistema = Sistema::new();
        sistema.cargar_venta(
            fecha.clone(),
            &cliente,
            &vendedor1,
            MediosDePago::Efectivo,
            vec![&producto1],
            cantidades1,
        );
        sistema.cargar_venta(
            fecha.clone(),
            &cliente,
            &vendedor2,
            MediosDePago::Efectivo,
            vec![&producto2, &producto3],
            cantidades2,
        );

        let reporte_vendedores = sistema.generar_reporte_por_vendedor();
        assert_eq!(*reporte_vendedores.get(&vendedor1).unwrap(), 1);
        assert_eq!(*reporte_vendedores.get(&vendedor2).unwrap(), 1);
    }

    // tests de validacion de fechas. Probado biciesto con febrero 29 en 2024. Y cuando no lo es. Y caso absurdo de 32 de mayo del xxxx
    #[test]
    fn test_es_fecha_valida_success_1() {
        let d = 28;
        let m = 5;
        let a = 2002;
        let f1 = Fecha::new(d, m, a);

        assert!(f1.es_fecha_valida());
    }
    #[test]
    fn test_es_fecha_valida_success_2() {
        let d = 29;
        let m = 2;
        let a = 2024;
        let f1 = Fecha::new(d, m, a);

        assert!(f1.es_fecha_valida());
    }

    #[test]
    fn test_es_fecha_valida_fail_1() {
        let d = 32;
        let m = 5;
        let a = 2002;
        let f1 = Fecha::new(d, m, a);

        assert!(!f1.es_fecha_valida());
    }

    #[test]
    fn test_es_fecha_valida_fail_2() {
        let d = 30;
        let m = 2;
        let a = 2002;
        let f1 = Fecha::new(d, m, a);

        assert!(!f1.es_fecha_valida());
    }

    // validaciones para la funcion de sumar dias. Casos de < 30 dias, >30, >100, >365.
    #[test]
    fn test_sumar_dias_pocos() {
        let d = 2;
        let m = 2;
        let a = 2024;
        let mut f1 = Fecha::new(d, m, a);
        f1.sumar_dias(12);

        let fecha_esperada = Fecha::new(14, 2, 2024);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_sumar_dias_mas_de_30() {
        let d = 5;
        let m = 5;
        let a = 2024;
        let mut f1 = Fecha::new(d, m, a);
        f1.sumar_dias(53);

        let fecha_esperada = Fecha::new(27, 6, 2024);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_sumar_dias_mas_de_100() {
        let d = 23;
        let m = 7;
        let a = 2000;
        let mut f1 = Fecha::new(d, m, a);
        f1.sumar_dias(144);

        let fecha_esperada = Fecha::new(14, 12, 2000);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_sumar_dias_mas_de_365() {
        let d = 14;
        let m = 11;
        let a = 1988;
        let mut f1 = Fecha::new(d, m, a);
        f1.sumar_dias(512);

        let fecha_esperada = Fecha::new(10, 4, 1990);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }

    // validaciones para la funcion de restar dias. Casos de < 30 dias, >30, >100, >365.
    #[test]
    fn test_restar_dias_pocos() {
        let d = 29;
        let m = 2;
        let a = 2024;
        let mut f1 = Fecha::new(d, m, a);
        f1.restar_dias(12);

        let fecha_esperada = Fecha::new(17, 2, 2024);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_restar_dias_mas_de_30() {
        let d = 5;
        let m = 5;
        let a = 2024;
        let mut f1 = Fecha::new(d, m, a);
        f1.restar_dias(53);

        let fecha_esperada = Fecha::new(13, 3, 2024);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_restar_dias_mas_de_100() {
        let d = 23;
        let m = 7;
        let a = 2000;
        let mut f1 = Fecha::new(d, m, a);
        f1.restar_dias(144);

        let fecha_esperada = Fecha::new(1, 3, 2000);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
    #[test]
    fn test_restar_dias_mas_de_365() {
        let d = 14;
        let m = 11;
        let a = 1988;
        let mut f1 = Fecha::new(d, m, a);
        f1.restar_dias(512);

        let fecha_esperada = Fecha::new(21, 6, 1987);

        assert_eq!(fecha_esperada.dia, f1.dia);
        assert_eq!(fecha_esperada.mes, f1.mes);
        assert_eq!(fecha_esperada.anio, f1.anio);
    }
}
