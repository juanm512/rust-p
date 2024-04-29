// 6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
// usuario ingresar un número entero por teclado para sumarse con la variable definida. El
// programa debe imprimir el valor del número elevado al cuadrado.

use std::io::stdin;

pub(crate) fn run(){
    let num1: u64;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    num1 = match input.trim().parse::<u64>() {
        Ok(valor) => valor,
        Err(_) => {
            println!("Error: Ingrese un valor numerico entero sin signo ( 0-255 ).");
            return;
        },
    };

    println!("{} ^2 = {}", num1, num1.pow(2));
}