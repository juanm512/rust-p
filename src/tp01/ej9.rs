// 9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
// suma de los valores del arreglo.

pub(crate) fn run(){
    let arr = [2, 54, 43, 675, 2];

    let mut suma = 0;
    for num in arr {
        suma += num;
    };

    println!("La suma es: {}", suma);

}