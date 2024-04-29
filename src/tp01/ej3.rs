// 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
// ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
// and y or. Se deben imprimir ambos resultados.

use std::io::stdin;

pub(crate) fn run() {
    
    let flag: bool = false; 

    // read a boolean value from keyboard and assign it to the variable flag2.
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let flag2 = match input.trim().parse::<bool>() {
        Ok(valor) => valor,
        Err(_) => {
            println!("Error: Ingrese un valor booleano (true o false).");
            return;
        },
    };

    println!("Operacion AND : {}", flag && !flag2);
    println!("Operacion OR : {}", flag || !flag2);
}
