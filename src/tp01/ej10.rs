// 10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
// cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
// originales.


pub(crate) fn run(){
    let arr1 = [2, 54, 43, 675, 2];
    let arr2 = [2, 54, 43, 675, 2];
    let mut arr3: [u32; 5] = [ 0, 0,0,0,0];

    for i in 0..arr1.len() {
        arr3[i] = arr1[i] + arr2[i];
    }

    print!("[ ");
    for num in arr3 {
        print!(" {} ", num);
    };
    print!(" ]\n");
}