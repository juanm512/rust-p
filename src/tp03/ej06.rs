// 6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
// un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
// arreglo.

pub fn run(){
    let cadena0 = String::from("elipee");
    let cadena1 = String::from("tres body's problem");
    let cadena2 = String::from("supernatural");
    

    let array_cadenas = [cadena0, cadena1, cadena2]; 
    println!("Siendo las cadenas: {:?}", array_cadenas);
    println!("Longitud de cada cadena es: {:?}", longitud_de_cadenas(&array_cadenas));
    println!("Siendo las cadenas: {:?}", array_cadenas);
}

fn longitud_de_cadenas( cadenas: &[String; 3] ) -> [usize;3]{
    let mut long = [0,0,0];

    let mut i = 0;
    for cadena in cadenas {
        long[i] = cadena.len();
        i += 1;
    }

    long
}

#[test]
fn test_longitud_de_cadenas(){
    let cadena0 = String::from("elipee");
    let cadena1 = String::from("tres body's problem");
    let cadena2 = String::from("supernatural");
    

    let array_cadenas = [cadena0, cadena1, cadena2]; 

    assert_eq!(longitud_de_cadenas(&array_cadenas),[6,19,12]);
}