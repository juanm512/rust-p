// 12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
// reemplaza todos los números pares por -1.

pub fn run(){
    let mut array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];

    println!("{:?}", array);
    reemplazar_pares(&mut array);
    println!("reemplazodos los pares: {:?}", array);
   
}

fn reemplazar_pares( array: &mut [i32; 8]) {

    for num in array {
        if *num % 2 == 0 {
            *num = -1;
        }
    }

}


// #[test]
// fn test_reemplazar_pares() { 
//     let mut array = [ 124, -144, 0, 10, 256, 33, 32, 76 ];
//     let factor = 5;
//     assert_eq!(10,array[3]);
//     reemplazar_pares(&mut array, factor);
//     assert_eq!(50,array[3]);
//  }