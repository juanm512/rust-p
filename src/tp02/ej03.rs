// 3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
// números enteros y retorna la suma de los números pares.

use crate::tp02::ej01::es_par;

pub fn run(){
    let arr = [1, 2, 3, 4, 5, 13];
    println!("Suma de los numeros pares nomas {:?} -> {:?}", arr, suma_pares(arr) );
}

fn suma_pares( arr: [i32; 6] ) -> i32{

    let mut sumatoria = 0;
    
    for i in 0..arr.len() { 
        sumatoria += if es_par(arr[i]) { arr[i] } else { 0 } ;
    }

    return sumatoria;
}


#[test]
pub fn test_suma_pares(){ 
    assert_eq!( suma_pares([1, 2, 3, 4, 5, 13]),  6 ) ;
    assert_ne!( suma_pares([1, 2, 3, 4, 5, 13]),  28 ) ;
 }