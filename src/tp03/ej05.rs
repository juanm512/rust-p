// 5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
// retorna un arreglo nuevo con los valores duplicados del parámetro.

pub fn run(){
    let mut array: [f32; 8] = [1.94, 24.8, 32.5, 14.0, 5.0, 6.23, 27.1, 85.6];
    println!("El arreglo original es {:?}", array);
    
    array = *duplicar_valores(&mut array);
    println!("El nuevo arreglo es {:?}", array);
}

fn duplicar_valores( numeros: & mut [f32; 8] ) -> &[f32; 8]{
    for i in 0..numeros.len()   {
        numeros[i] = numeros[i]*2.0;
    }

   numeros
}

#[test]
fn test_duplicar_valores(){
    let mut array: [f32; 8] = [1.94, 24.8, 32.5, 14.0, 5.0, 6.23, 27.1, 85.6];
    array = *duplicar_valores(&mut array);

    let array_duplicado: [f32; 8] = [3.88, 49.6, 65.0, 28.0, 10.0, 12.46, 54.2, 171.2];
    assert_eq!(array, array_duplicado);
}