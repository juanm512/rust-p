// 1- Escribir un programa que defina una variable de tipo flotante 
// con algún valor, y luego
// permita al usuario ingresar un número decimal por teclado
// para multiplicar, dividir, sumar y restar su valor. 
// Se deben imprimir los resultados.

pub fn run() {
    let float_num1: f64 = 5.3;
    
    // println!("Ingrese un número decimal por teclado: ");
    // let mut string_num2 = String::new();
    // stdin().read_line(&mut string_num2).expect("Error al leer el numero.");
    let float_num2: f64 = 1.365;

    println!("{} * {} = {}", float_num1, float_num2, (float_num1 * float_num2));
    println!("{} / {} = {}", float_num1, float_num2, (float_num1 / float_num2));
    println!("{} + {} = {}", float_num1, float_num2, (float_num1 + float_num2));
    println!("{} - {} = {}", float_num1, float_num2, (float_num1 - float_num2));
   }