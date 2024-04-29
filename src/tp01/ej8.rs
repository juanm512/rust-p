// 8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
// número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
// Se debe imprimir el resultado.

use std::io::stdin;


pub(crate) fn run(){
    let string_constant: String = String::from("Escribir un programa que defina una constante de tipo cadena");
    
    println!("cadena a realizar la busqueda: '{}'",string_constant);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let char = match input.trim().parse::<char>() {
        Ok(valor) => valor,
        Err(_) => {
            println!("Error: Ingrese un caracter.");
            return;
        },
    };

    println!("- caracter a buscar: {}",input);

    let mut sum = 0;
    
    for character in string_constant.to_lowercase().chars() {
        if  character == char { sum += 1; }
    }

    println!("- numero de veces que el caracter aparece en la cadena: '{}'",sum);
}