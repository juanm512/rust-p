// 9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
// enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
// función retorna la cantidad de números del arreglo que están entre el rango de los
// parámetros inferior y superior inclusive.

pub fn run(){
    let array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];
    let min = -5;
    let max = 100;

    println!("La cantidad de números del arreglo que están entre {} y {} es: {}", min, max, cantidad_en_rango(array, min, max));
}

fn cantidad_en_rango( array: [i32; 8], min: i32, max: i32) -> i32{
    let mut cantidad = 0;

    for num  in array {
        if num >= min && num <= max { cantidad += 1 }
    }

    cantidad
}


#[test]
fn test_cantidad_en_rango() { 
    let array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];
    let min = -5;
    let max = 100;

    assert_eq!(5,cantidad_en_rango(array, min, max) )
 }