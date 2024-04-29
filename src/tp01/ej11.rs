// 11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
// ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
// ingresada por el usuario se encuentra en el arreglo

use std::io::stdin;

pub(crate) fn run(){
    let arr = [
        "hola","mundo","este","es","rust"
    ];

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    println!("el arreglo contiene la cadena '{}': {}", input.trim(), arr.contains(&input.as_str().trim()) );
}