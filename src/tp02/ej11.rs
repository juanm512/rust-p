// 11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
// enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
// por el parámetro factor modificándolo.

pub fn run(){
    let mut array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];
    let factor = 5;


    println!("{:?}", array);
    multiplicar_valores(&mut array, factor);

    println!("x {}", factor);

    println!("---------------------------------------");
    println!("{:?}", array);
}

fn multiplicar_valores( array: &mut [i32; 8], factor: i32) {


    for i in 0..array.len() {
        array[i] *= factor;
    }

}


#[test]
fn test_multiplicar_valores() { 
    let mut array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];
    let factor = 5;
    assert_eq!(10,array[3]);
    multiplicar_valores(&mut array, factor);
    assert_eq!(50,array[3]);
 }