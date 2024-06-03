// 3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
// (Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
// duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
// suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
// Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
// correspondientes a excepción de Efectivo.
// Los usuarios solo pueden tener una suscripción activa a la vez.

// Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
// siguientes acciones:
// ➢ Crear un usuario con una determinada suscripción y medio de pago.
// ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
// pasa a Clasic y si está en Clasic pasa a Super.
// ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
// suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
// ➢ Dado un usuario cancelar la suscripción.

// ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
// suscripciones activas
// ➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
// activas.
// ➢ Saber cuál fue el medio de pago más utilizado.
// ➢ Saber cuál fue la suscripción más contratada.

use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};

// /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MediosDePago {
    Efectivo,
    MercadoPago(MercadoPagoData),
    TarjetaDeCredito(TarjetaDeCreditoData),
    TransferenciaBancaria(TransferenciaBancariaData),
    Cripto(CriptoData),
}

impl MediosDePago {
    pub fn clone_sin_data(&self) -> Self {
        match self {
            MediosDePago::Efectivo => MediosDePago::Efectivo,
            MediosDePago::MercadoPago(_) => MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "".to_string(),
                transaction_id: "".to_string(),
            }),
            MediosDePago::TarjetaDeCredito(_) => {
                MediosDePago::TarjetaDeCredito(TarjetaDeCreditoData {
                    card_number: "".to_string(),
                    card_holder_name: "".to_string(),
                    expiration_date: "".to_string(),
                    cvv: "".to_string(),
                })
            }
            MediosDePago::TransferenciaBancaria(_) => {
                MediosDePago::TransferenciaBancaria(TransferenciaBancariaData {
                    bank_name: "".to_string(),
                    account_number: "".to_string(),
                    routing_number: "".to_string(),
                })
            }
            MediosDePago::Cripto(_) => MediosDePago::Cripto(CriptoData {
                wallet_address: "".to_string(),
                transaction_hash: "".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MercadoPagoData {
    account_id: String,
    transaction_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TarjetaDeCreditoData {
    card_number: String,
    card_holder_name: String,
    expiration_date: String,
    cvv: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TransferenciaBancariaData {
    bank_name: String,
    account_number: String,
    routing_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CriptoData {
    wallet_address: String,
    transaction_hash: String,
}

// /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

#[derive(Debug, Clone, PartialEq)]
enum EstadoSuscripcion {
    Activa,
    Cancelada,
}

#[derive(Debug, Clone, PartialEq)]
struct Suscripcion {
    costo: f64,
    duracion: u8,
    fecha_inicio: DateTime<Utc>,
    tipo: TipoSuscripcion,
    id_usuario: u32,
    estado: EstadoSuscripcion,
}

// /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\

#[derive(Debug, Clone, PartialEq)]
struct Usuario {
    id: u32,
    nombre_apellido: String,
    email: String,
    metodo_pago: MediosDePago,
}

// /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\ /\/\/\/\/\/\/\/\/\/\/\/\/\
#[derive(Debug)]
struct StreamingRust {
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>,
}

impl StreamingRust {
    fn new() -> Self {
        StreamingRust {
            usuarios: Vec::new(),
            suscripciones: Vec::new(),
        }
    }

    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }

    fn agregar_suscripcion(&mut self, suscripcion: Suscripcion) {
        self.suscripciones.push(suscripcion);
    }

    fn crear_usuario_con_suscripcion(
        &mut self,
        id: u32,
        nombre_apellido: String,
        email: String,
        metodo_pago: MediosDePago,
        tipo_suscripcion: TipoSuscripcion,
        costo: f64,
        duracion: u8,
        fecha_inicio: DateTime<Utc>,
    ) -> bool {
        if self.usuarios.iter().any(|u| u.id == id) {
            println!("El usuario con id {} ya existe en el sistema", id);
            return false;
        }

        let suscripcion = Suscripcion {
            costo,
            duracion,
            fecha_inicio,
            tipo: tipo_suscripcion,
            id_usuario: id,
            estado: EstadoSuscripcion::Activa,
        };

        let usuario = Usuario {
            id,
            nombre_apellido,
            email,
            metodo_pago,
        };

        self.agregar_usuario(usuario);
        self.agregar_suscripcion(suscripcion);
        return true;
    }

    fn upgrade_suscripcion(&mut self, id_usuario: u32) {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|s| s.id_usuario == id_usuario)
        {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Basic => TipoSuscripcion::Clasic,
                TipoSuscripcion::Clasic => TipoSuscripcion::Super,
                TipoSuscripcion::Super => TipoSuscripcion::Super,
            };
        }
    }

    fn downgrade_suscripcion(&mut self, id_usuario: u32) {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|s| s.id_usuario == id_usuario)
        {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Super => TipoSuscripcion::Clasic,
                TipoSuscripcion::Clasic => TipoSuscripcion::Basic,
                TipoSuscripcion::Basic => {
                    suscripcion.estado = EstadoSuscripcion::Cancelada;
                    return;
                }
            };
        }
    }

    fn cancelar_suscripcion(&mut self, id_usuario: u32) {
        if let Some(suscripcion) = self
            .suscripciones
            .iter_mut()
            .find(|s| s.id_usuario == id_usuario)
        {
            suscripcion.estado = EstadoSuscripcion::Cancelada;
        }
    }

    fn metodo_pago_mas_usado_susc_activas(&self) -> Option<MediosDePago> {
        let mut contador_metodos_pago = HashMap::new();
        for susc in self
            .suscripciones
            .iter()
            .filter(|s| s.estado == EstadoSuscripcion::Activa)
        {
            if let Some(usuario) = self.usuarios.iter().find(|u| u.id == susc.id_usuario) {
                *contador_metodos_pago
                    .entry(usuario.metodo_pago.clone_sin_data())
                    .or_insert(0) += 1;
            }
        }
        contador_metodos_pago
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tipo, _)| tipo.clone_sin_data())
    }

    fn susc_mas_usada_activas(&self) -> Option<TipoSuscripcion> {
        let mut contador_tipo_susc = HashMap::new();
        for susc in self
            .suscripciones
            .iter()
            .filter(|s| s.estado == EstadoSuscripcion::Activa)
        {
            *contador_tipo_susc.entry(&susc.tipo).or_insert(0) += 1;
        }
        contador_tipo_susc
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tipo, _)| tipo.clone())
    }

    fn metodo_pago_mas_usado(&self) -> Option<MediosDePago> {
        let mut contador_metodos_pago = HashMap::new();
        for susc in self.suscripciones.iter() {
            if let Some(usuario) = self.usuarios.iter().find(|u| u.id == susc.id_usuario) {
                *contador_metodos_pago
                    .entry(usuario.metodo_pago.clone_sin_data())
                    .or_insert(0) += 1;
            }
        }
        // println!("{:#?}", contador_metodos_pago);
        contador_metodos_pago
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tipo, _)| tipo.clone_sin_data())
    }

    fn susc_mas_usada(&self) -> Option<TipoSuscripcion> {
        let mut contador_tipo_susc = HashMap::new();
        for susc in self.suscripciones.iter() {
            *contador_tipo_susc.entry(&susc.tipo).or_insert(0) += 1;
        }
        contador_tipo_susc
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tipo, _)| tipo.clone())
    }
}
//
//
//

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data() -> StreamingRust {
        let mut plataforma = StreamingRust::new();

        let fecha_inicio = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);

        plataforma.crear_usuario_con_suscripcion(
            1,
            "Juan Perez".to_string(),
            "juan.perez@example.com".to_string(),
            MediosDePago::Efectivo,
            TipoSuscripcion::Basic,
            100.0,
            1,
            fecha_inicio,
        );

        plataforma.crear_usuario_con_suscripcion(
            2,
            "Maria Gomez".to_string(),
            "maria.gomez@example.com".to_string(),
            MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "mp_123".to_string(),
                transaction_id: "tx_456".to_string(),
            }),
            TipoSuscripcion::Clasic,
            200.0,
            1,
            fecha_inicio,
        );

        plataforma.crear_usuario_con_suscripcion(
            3,
            "Pedro Lopez".to_string(),
            "pedro.lopez@example.com".to_string(),
            MediosDePago::TarjetaDeCredito(TarjetaDeCreditoData {
                card_number: "1234-5678-9876-5432".to_string(),
                card_holder_name: "Pedro Lopez".to_string(),
                expiration_date: "12/25".to_string(),
                cvv: "123".to_string(),
            }),
            TipoSuscripcion::Super,
            300.0,
            2,
            fecha_inicio,
        );

        plataforma.crear_usuario_con_suscripcion(
            4,
            "Mateo Gutierrez".to_string(),
            "mateo.guz@example.com".to_string(),
            MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "mp_422".to_string(),
                transaction_id: "tx_422".to_string(),
            }),
            TipoSuscripcion::Clasic,
            200.0,
            6,
            fecha_inicio,
        );
        plataforma.crear_usuario_con_suscripcion(
            5,
            "Lautaro Sanches".to_string(),
            "lautaro.s@example.com".to_string(),
            MediosDePago::TarjetaDeCredito(TarjetaDeCreditoData {
                card_number: "1234-5678-9876-5432".to_string(),
                card_holder_name: "Lautaro Sanches".to_string(),
                expiration_date: "12/27".to_string(),
                cvv: "221".to_string(),
            }),
            TipoSuscripcion::Clasic,
            200.0,
            3,
            fecha_inicio,
        );
        plataforma.crear_usuario_con_suscripcion(
            6,
            "Ignacio Alvarez".to_string(),
            "ign.alva@example.com".to_string(),
            MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "mp_563".to_string(),
                transaction_id: "tx_563".to_string(),
            }),
            TipoSuscripcion::Super,
            300.0,
            1,
            fecha_inicio,
        );

        plataforma
    }

    #[test]
    fn test_crear_usuario_con_suscripcion() {
        let mut plataforma = StreamingRust::new();
        let fecha_inicio = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);

        plataforma.crear_usuario_con_suscripcion(
            1,
            "Juan Perez".to_string(),
            "juan.perez@example.com".to_string(),
            MediosDePago::Efectivo,
            TipoSuscripcion::Basic,
            100.0,
            1,
            fecha_inicio,
        );
        let se_pudo_crear = plataforma.crear_usuario_con_suscripcion(
            1,
            "Juan Perez".to_string(),
            "juan.perez@example.com".to_string(),
            MediosDePago::Efectivo,
            TipoSuscripcion::Basic,
            100.0,
            1,
            fecha_inicio,
        );
        assert_ne!(true, se_pudo_crear);

        assert_eq!(plataforma.usuarios.len(), 1);
        assert_eq!(plataforma.suscripciones.len(), 1);
    }

    #[test]
    fn test_upgrade_suscripcion() {
        let mut plataforma = create_test_data();
        plataforma.upgrade_suscripcion(1);

        let suscripcion = plataforma
            .suscripciones
            .iter()
            .find(|s| s.id_usuario == 1)
            .unwrap();

        assert_eq!(suscripcion.tipo, TipoSuscripcion::Clasic);
    }

    #[test]
    fn test_downgrade_suscripcion() {
        let mut plataforma = create_test_data();
        plataforma.downgrade_suscripcion(2);

        let suscripcion = plataforma
            .suscripciones
            .iter()
            .find(|s| s.id_usuario == 2)
            .unwrap();

        assert_eq!(suscripcion.tipo, TipoSuscripcion::Basic);
    }

    #[test]
    fn test_cancelar_suscripcion() {
        let mut plataforma = create_test_data();
        plataforma.cancelar_suscripcion(1);

        let suscripcion = plataforma
            .suscripciones
            .iter()
            .find(|s| s.id_usuario == 1)
            .unwrap();

        assert_eq!(suscripcion.estado, EstadoSuscripcion::Cancelada);
    }

    #[test]
    fn test_metodo_pago_mas_usado_susc_activas() {
        let mut plataforma = create_test_data();
        plataforma.cancelar_suscripcion(5);
        let metodo_mas_usado = plataforma.metodo_pago_mas_usado_susc_activas();

        assert_eq!(
            metodo_mas_usado,
            Some(MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "".to_string(),
                transaction_id: "".to_string()
            }))
        );
    }

    #[test]
    fn test_susc_mas_usada_activas() {
        let mut plataforma = create_test_data();
        plataforma.cancelar_suscripcion(1);
        let suscripcion_mas_usada = plataforma.susc_mas_usada_activas();

        assert_eq!(suscripcion_mas_usada, Some(TipoSuscripcion::Clasic));
    }

    #[test]
    fn test_metodo_pago_mas_usado() {
        let plataforma = create_test_data();
        let metodo_mas_usado = plataforma.metodo_pago_mas_usado();

        assert_eq!(
            Some(MediosDePago::MercadoPago(MercadoPagoData {
                account_id: "".to_string(),
                transaction_id: "".to_string()
            })),
            metodo_mas_usado
        );
    }

    #[test]
    fn test_susc_mas_usada() {
        let plataforma = create_test_data();
        let suscripcion_mas_usada = plataforma.susc_mas_usada();

        assert_eq!(suscripcion_mas_usada, Some(TipoSuscripcion::Clasic));
    }
}
