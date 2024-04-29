// 4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
// números enteros y retorna la cantidad de números impares.

use crate::tp02::ej01::es_par;

pub fn run(){
    let arr = [1, 2, 3, 4, 5, 13];
    println!("Suma de los numeros pares nomas {:?} -> {:?}", arr, cantidad_impares(arr) );
}

fn cantidad_impares( arr: [i32; 6] ) -> i32{

    let mut cant = 0;
    
    for i in 0..arr.len() { 
        cant += if es_par(arr[i]) { 0 } else { 1 } ;
    }

    return cant;
}


#[test]
pub fn test_cantidad_impares(){ 
    assert_eq!( cantidad_impares([1, 2, 3, 4, 5, 13]),  4 ) ;
    assert_ne!( cantidad_impares([1, 2, 3, 4, 5, 13]),  28 ) ;
 }