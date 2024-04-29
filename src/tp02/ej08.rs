// 8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
// números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
// arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
// arreglos pasados por parámetro.

pub fn run(){
    let array1: [f32; 8] = [1.494, 24.853, 32.25, 14.06, 52.4, 32.23, 57.6, 87.78];
    let array2: [f32; 8] = [12.94, 5.56, 53.5, 16.3, 13.0, 6.23, 27.1, 85.6];

    println!("{:?}", array1);
    println!("+");
    println!("{:?}", &array2);
    println!("----------------------------------------------------");
    println!("{:?}", sumar_arreglos(&array1, &array2));
}

fn sumar_arreglos(arr1: &[f32; 8], arr2: &[f32; 8]) -> [f32; 8]{
    let mut new_arr = [0.0; 8];

    for i in 0..arr1.len() { 
        new_arr[i] = arr1[i] + arr2[i];
    }

    new_arr
}

#[test]
fn test_sumar_arreglos() {
    let array1: [f32; 8] = [1.494, 24.853, 32.25, 14.06, 52.4, 32.23, 57.6, 87.78];
    let array2: [f32; 8] = [12.94, 5.56, 53.5, 16.3, 13.0, 6.23, 27.1, 85.6];

    assert_eq!(14.434, sumar_arreglos(&array1, &array2)[0]);
}