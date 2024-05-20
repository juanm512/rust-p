// 1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
// números primos. Cree un trait para la determinación del número primo e impleméntelo
// según corresponda. Utilice la función iter sobre el vector y aplique un closure para
// resolverlo.

pub trait NumPrimo {
    fn es_primo(&self) -> bool;
}

impl NumPrimo for u64 {
    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false;
        }
        for i in 2..=(*self as f64).sqrt() as u64 {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}

fn cantidad_primos(vec: Vec<u64>) -> u64 {
    vec.iter().filter(|x| x.es_primo()).count() as u64
}

pub fn run() {
    let vec = Vec::from([2, 6, 7, 8, 5, 17, 18]);
    println!(
        "cantidad de numeros primos en {:#?}: {}",
        vec,
        cantidad_primos(vec.clone())
    );
}
