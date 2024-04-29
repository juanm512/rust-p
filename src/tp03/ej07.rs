// 7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
// de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
// números mayores al límite que tiene el arreglo.


pub fn run(){
    let arr = [1, 25, 12, 2, 5, 51, 52, 512];
    let limite = 50;

    println!("Cantidad de numeros en el array mayores a {}: {}", limite, cantidad_de_mayores(&arr, limite))
}

fn cantidad_de_mayores(arr: &[i32; 8], limite: i32) -> i32{
    let mut cantidad = 0;

    for numero in arr { 
        if numero  > &limite { // Si el número es mayor al límite, se incrementa
            cantidad += 1;
        }
    }

    cantidad
}

#[test]
fn test_cantidad_de_mayores() {
    let arr = [1, 25, 12, 2, 5, 51, 52, 512];
    let limite = 50;

    assert_eq!(3, cantidad_de_mayores(&arr, limite));
}