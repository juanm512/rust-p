// 5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
// ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
// cadena en mayúsculas.

use std::io::stdin;

pub(crate) fn run(){
    let mut cadena = String::from("HOLA, ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    cadena += &input;

    println!("La cadena en mayusculas es: {}", cadena.to_uppercase())
}