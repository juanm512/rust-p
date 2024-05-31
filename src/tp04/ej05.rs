use chrono::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct User {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validado: bool,
    balance_fiat: f64,
    balance_cripto: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    blockchains: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Debug, Clone, PartialEq)]
enum TransaccionTipo {
    IngresoDinero,
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    RetiroFiat,
}

#[derive(Debug, Clone, PartialEq)]
struct Transaccion {
    fecha: DateTime<Utc>,
    tipo: TransaccionTipo,
    monto: f64,
    usuario: String,
    cripto: Option<String>,
    cotizacion: Option<f64>,
    blockchain: Option<String>,
    hash: Option<String>,
    medio: Option<String>,
}

struct Plataforma {
    usuarios: HashMap<String, User>,
    criptomonedas: HashMap<String, Criptomoneda>,
    blockchains: HashMap<String, Blockchain>,
    transacciones: Vec<Transaccion>,
}

impl Plataforma {
    fn new() -> Self {
        Plataforma {
            usuarios: HashMap::new(),
            criptomonedas: HashMap::new(),
            blockchains: HashMap::new(),
            transacciones: Vec::new(),
        }
    }

    fn agregar_usuario(&mut self, user: User) {
        self.usuarios.insert(user.email.clone(), user);
    }

    fn agregar_blockchain(&mut self, blockchain: Blockchain) {
        self.blockchains
            .insert(blockchain.nombre.clone(), blockchain);
    }

    fn obtener_blockchains(&self, nombres_blockchains: Vec<String>) -> Vec<&Blockchain> {
        nombres_blockchains
            .iter()
            .filter_map(|name| self.blockchains.get(name))
            .collect()
    }

    fn agregar_criptomoneda(&mut self, cripto: Criptomoneda) {
        let nombre = cripto.nombre.clone();
        self.criptomonedas.insert(nombre, cripto);
    }

    fn obtener_usuario(&self, email: &str) -> Option<&User> {
        self.usuarios.get(email)
    }

    fn obtener_usuario_mut(&mut self, email: &str) -> Option<&mut User> {
        self.usuarios.get_mut(email)
    }

    fn ingresar_dinero(&mut self, email: &str, monto: f64) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            user.balance_fiat += monto;
            self.transacciones.push(Transaccion {
                fecha: Utc::now(),
                tipo: TransaccionTipo::IngresoDinero,
                monto,
                usuario: email.to_string(),
                cripto: None,
                cotizacion: None,
                blockchain: None,
                hash: None,
                medio: None,
            });
        }
    }

    fn comprar_cripto(
        &mut self,
        email: &str,
        cripto_nombre: &str,
        monto_fiat: f64,
        cotizacion: f64,
    ) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            if user.validado && user.balance_fiat >= monto_fiat {
                let monto_cripto = monto_fiat / cotizacion;
                user.balance_fiat -= monto_fiat;
                *user
                    .balance_cripto
                    .entry(cripto_nombre.to_string())
                    .or_insert(0.0) += monto_cripto;
                self.transacciones.push(Transaccion {
                    fecha: Utc::now(),
                    tipo: TransaccionTipo::CompraCripto,
                    monto: monto_fiat,
                    usuario: email.to_string(),
                    cripto: Some(cripto_nombre.to_string()),
                    cotizacion: Some(cotizacion),
                    blockchain: None,
                    hash: None,
                    medio: None,
                });
            }
        }
    }

    fn vender_cripto(
        &mut self,
        email: &str,
        cripto_nombre: &str,
        monto_cripto: f64,
        cotizacion: f64,
    ) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            if user.validado {
                if let Some(balance_cripto) = user.balance_cripto.get_mut(cripto_nombre) {
                    if *balance_cripto >= monto_cripto {
                        let monto_fiat = monto_cripto * cotizacion;
                        *balance_cripto -= monto_cripto;
                        user.balance_fiat += monto_fiat;
                        self.transacciones.push(Transaccion {
                            fecha: Utc::now(),
                            tipo: TransaccionTipo::VentaCripto,
                            monto: monto_cripto,
                            usuario: email.to_string(),
                            cripto: Some(cripto_nombre.to_string()),
                            cotizacion: Some(cotizacion),
                            blockchain: None,
                            hash: None,
                            medio: None,
                        });
                    }
                }
            }
        }
    }

    fn retirar_cripto(
        &mut self,
        email: &str,
        cripto_nombre: &str,
        monto_cripto: f64,
        blockchain_nombre: &str,
    ) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            if user.validado {
                if let Some(balance_cripto) = user.balance_cripto.get_mut(cripto_nombre) {
                    if *balance_cripto >= monto_cripto {
                        *balance_cripto -= monto_cripto;
                        let hash =
                            format!("{}-{}", blockchain_nombre, rand::thread_rng().gen::<u64>());
                        self.transacciones.push(Transaccion {
                            fecha: Utc::now(),
                            tipo: TransaccionTipo::RetiroCripto,
                            monto: monto_cripto,
                            usuario: email.to_string(),
                            cripto: Some(cripto_nombre.to_string()),
                            cotizacion: None,
                            blockchain: Some(blockchain_nombre.to_string()),
                            hash: Some(hash),
                            medio: None,
                        });
                    }
                }
            }
        }
    }

    fn recibir_cripto(
        &mut self,
        email: &str,
        cripto_nombre: &str,
        monto_cripto: f64,
        blockchain_nombre: &str,
        cotizacion: f64,
    ) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            *user
                .balance_cripto
                .entry(cripto_nombre.to_string())
                .or_insert(0.0) += monto_cripto;
            self.transacciones.push(Transaccion {
                fecha: Utc::now(),
                tipo: TransaccionTipo::RecepcionCripto,
                monto: monto_cripto,
                usuario: email.to_string(),
                cripto: Some(cripto_nombre.to_string()),
                cotizacion: Some(cotizacion),
                blockchain: Some(blockchain_nombre.to_string()),
                hash: None,
                medio: None,
            });
        }
    }

    fn retirar_fiat(&mut self, email: &str, monto: f64, medio: &str) {
        if let Some(user) = self.obtener_usuario_mut(email) {
            if user.validado && user.balance_fiat >= monto {
                user.balance_fiat -= monto;
                self.transacciones.push(Transaccion {
                    fecha: Utc::now(),
                    tipo: TransaccionTipo::RetiroFiat,
                    monto,
                    usuario: email.to_string(),
                    cripto: None,
                    cotizacion: None,
                    blockchain: None,
                    hash: None,
                    medio: Some(medio.to_string()),
                });
            }
        }
    }

    fn cripto_mas_vendida(&self) -> Option<String> {
        let mut ventas: HashMap<String, u32> = HashMap::new();
        for tx in &self.transacciones {
            if let TransaccionTipo::VentaCripto = tx.tipo {
                if let Some(cripto) = &tx.cripto {
                    *ventas.entry(cripto.clone()).or_insert(0) += 1;
                }
            }
        }
        ventas
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(cripto, _)| cripto)
    }

    fn cripto_mas_comprada(&self) -> Option<String> {
        let mut compras: HashMap<String, u32> = HashMap::new();
        for tx in &self.transacciones {
            if let TransaccionTipo::CompraCripto = tx.tipo {
                if let Some(cripto) = &tx.cripto {
                    *compras.entry(cripto.clone()).or_insert(0) += 1;
                }
            }
        }
        compras
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(cripto, _)| cripto)
    }

    fn volumen_ventas(&self) -> Option<String> {
        let mut volumen: HashMap<String, f64> = HashMap::new();
        for tx in &self.transacciones {
            if let TransaccionTipo::VentaCripto = tx.tipo {
                if let Some(cripto) = &tx.cripto {
                    *volumen.entry(cripto.clone()).or_insert(0.0) += tx.monto;
                }
            }
        }
        volumen
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(cripto, _)| cripto)
    }

    fn volumen_compras(&self) -> Option<String> {
        let mut volumen: HashMap<String, f64> = HashMap::new();
        for tx in &self.transacciones {
            if let TransaccionTipo::CompraCripto = tx.tipo {
                if let Some(cripto) = &tx.cripto {
                    *volumen.entry(cripto.clone()).or_insert(0.0) += tx.monto;
                }
            }
        }
        volumen
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(cripto, _)| cripto)
    }
}

pub fn run() {
    // Ejemplo de uso de la plataforma
    let mut plataforma = Plataforma::new();

    let user = User {
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        email: "juan.perez@example.com".to_string(),
        dni: "12345678".to_string(),
        validado: true,
        balance_fiat: 1000.0,
        balance_cripto: HashMap::new(),
    };

    let blockchain = Blockchain {
        nombre: "Ethereum".to_string(),
        prefijo: "ETH".to_string(),
    };

    plataforma.agregar_blockchain(blockchain);

    plataforma.agregar_criptomoneda(Criptomoneda {
        nombre: "Bitcoin".to_string(),
        prefijo: "BTC".to_string(),
        blockchains: vec!["Ethereum".to_string()],
    });

    plataforma.agregar_usuario(user);

    plataforma.ingresar_dinero("juan.perez@example.com", 500.0);

    plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);

    plataforma.vender_cripto("juan.perez@example.com", "Bitcoin", 0.01, 50000.0);

    let cripto_mas_vendida = plataforma.cripto_mas_vendida();
    let cripto_mas_comprada = plataforma.cripto_mas_comprada();
    let volumen_ventas = plataforma.volumen_ventas();
    let volumen_compras = plataforma.volumen_compras();

    println!("Criptomoneda más vendida: {:?}", cripto_mas_vendida);
    println!("Criptomoneda más comprada: {:?}", cripto_mas_comprada);
    println!("Volumen de ventas: {:?}", volumen_ventas);
    println!("Volumen de compras: {:?}", volumen_compras);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn crear_usuario_valido() -> User {
        User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        }
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        plataforma.agregar_usuario(user);

        plataforma.ingresar_dinero("juan.perez@example.com", 500.0);
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(user.balance_fiat, 1500.0);
    }

    #[test]
    fn test_comprar_cripto() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(user.balance_fiat, 500.0);
        assert_eq!(*user.balance_cripto.get("Bitcoin").unwrap(), 0.01);
    }

    #[test]
    fn test_vender_cripto() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);
        plataforma.vender_cripto("juan.perez@example.com", "Bitcoin", 0.01, 50000.0);
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(user.balance_fiat, 1000.0);
        assert_eq!(*user.balance_cripto.get("Bitcoin").unwrap(), 0.0);
    }

    #[test]
    fn test_retirar_cripto() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);
        plataforma.retirar_cripto("juan.perez@example.com", "Bitcoin", 0.01, "Ethereum");
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(*user.balance_cripto.get("Bitcoin").unwrap(), 0.0);
    }

    #[test]
    fn test_recibir_cripto() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.recibir_cripto(
            "juan.perez@example.com",
            "Bitcoin",
            0.01,
            "Ethereum",
            50000.0,
        );
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(*user.balance_cripto.get("Bitcoin").unwrap(), 0.01);
    }

    #[test]
    fn test_retirar_fiat() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };

        plataforma.agregar_usuario(user);
        plataforma.retirar_fiat("juan.perez@example.com", 500.0, "Transferencia Bancaria");
        let user = plataforma
            .obtener_usuario("juan.perez@example.com")
            .unwrap();
        assert_eq!(user.balance_fiat, 500.0);
    }

    #[test]
    fn test_cripto_mas_vendida() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);
        plataforma.vender_cripto("juan.perez@example.com", "Bitcoin", 0.01, 50000.0);

        assert_eq!(plataforma.cripto_mas_vendida().unwrap(), "Bitcoin");
    }

    #[test]
    fn test_cripto_mas_comprada() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };
        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user);

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);
        plataforma.comprar_cripto("juan.perez@example.com", "Bitcoin", 500.0, 50000.0);

        assert_eq!(plataforma.cripto_mas_comprada().unwrap(), "Bitcoin");
    }
    fn test_volumen_ventas() {
        let mut plataforma = Plataforma::new();
        let user = User {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            email: "juan.perez@example.com".to_string(),
            dni: "12345678".to_string(),
            validado: true,
            balance_fiat: 1000.0,
            balance_cripto: HashMap::new(),
        };

        let blockchain = Blockchain {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
        };

        plataforma.agregar_blockchain(blockchain);
        plataforma.agregar_usuario(user.clone());

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });

        plataforma.comprar_cripto(&user.email, "Wrapped Bitcoin", 500.0, 50000.0);
        plataforma.vender_cripto(&user.email, "Wrapped Bitcoin", 0.01, 50000.0);

        assert_eq!(plataforma.volumen_ventas().unwrap(), "Bitcoin");
    }

    #[test]
    fn test_volumen_compras() {
        let mut plataforma = Plataforma::new();
        let user = crear_usuario_valido();

        plataforma.agregar_criptomoneda(Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec!["Ethereum".to_string()],
        });
        plataforma.agregar_usuario(user.clone());

        plataforma.comprar_cripto(&user.email, "Wrapped Bitcoin", 500.0, 50000.0);

        assert_eq!(plataforma.volumen_compras().unwrap(), "Bitcoin");
    }
}
