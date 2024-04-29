// 10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
// un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
// del arreglo que son de longitud mayor al parámetro límite.

pub fn run(){
    let cadenas = [String::from("The three bodys problem"), String::from("juan"), String::from("chernobyl")];
    let limite = 5;


    println!("Cantidad de cadenas que tienen longitud mayor a {} es: {}", limite, cantidad_de_cadenas_mayor_a(cadenas, limite) );
}

fn cantidad_de_cadenas_mayor_a( cadenas: [String; 3], limite: usize) -> i32{
    let mut cantidad = 0;

    for cadena  in cadenas {
        if cadena.len() >= limite { cantidad += 1 }
    }

    cantidad
}


#[test]
fn test_cantidad_de_cadenas_mayor_a() { 
    let cadenas = [String::from("The three bodys problem"), String::from("juan"), String::from("chernobyl")];
    let limite = 5;

    assert_eq!(2, cantidad_de_cadenas_mayor_a(cadenas, limite));
 }