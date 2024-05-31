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

// pub fn run() {
//     let vec = Vec::from([2, 6, 7, 8, 5, 17, 18]);
//     println!(
//         "cantidad de numeros primos en {:#?}: {}",
//         vec,
//         cantidad_primos(vec.clone())
//     );
// }
#[test]
fn test_es_primo() {
    assert!(2.es_primo()); // 2 is prime
    assert!(3.es_primo()); // 3 is prime
    assert!(!4.es_primo()); // 4 is not prime
    assert!(5.es_primo()); // 5 is prime
    assert!(!6.es_primo()); // 6 is not prime
    assert!(7.es_primo()); // 7 is prime
    assert!(!8.es_primo()); // 8 is not prime
}

#[test]
fn test_cantidad_primos() {
    let vec = vec![2, 4, 5, 6, 7, 8, 9];
    assert_eq!(cantidad_primos(vec), 3);

    let vec = vec![2, 3, 4, 5, 6];
    assert_eq!(cantidad_primos(vec), 3);

    let vec = vec![10, 11, 12, 13, 14, 15];
    assert_eq!(cantidad_primos(vec), 2);

    let vec = vec![20, 30, 40, 50, 60];
    assert_eq!(cantidad_primos(vec), 0);
}
