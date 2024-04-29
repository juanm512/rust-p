// 4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
// con signo y un valor booleano, y luego imprima cada valor de la tupla

pub(crate) fn run() {
    let tupla = ("quinientos doce", 512, true);
    println!("La cadena es: {}", tupla.0);
    println!("El entero es: {}", tupla.1);
    println!("El booleano es: {}", tupla.2);
}
