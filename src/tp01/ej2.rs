// 2- Escribir un programa que defina una variable de tipo entero sin signo, 
// y luego imprima su valor en hexadecimal.

// use std::io::stdin;


pub(crate) fn run() {
    
    let x: u8 = 42; // 42 is '2a' in hex

    println!("{}", format!("{x:x}"));
    println!("{}", format!("{x:#x}"));


    // println!("\n\n---- Presione la tecla <Enter> para Salir del Programa ----");
    // let mut string = String::new();
    // stdin().read_line( &mut string).expect("Error al cerrar archivo.");
}
