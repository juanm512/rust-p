#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::stdin;

// mod tp01;
// mod tp02;
mod tp03;
// mod entregable_1;


fn main() {
    // tp01::ej1::run();
    // tp02::ej06::run();
    tp03::ej04::run();




    // let names: [String;8] = [ String::from("azucar"), String::from("arroz"), String::from("fideos"), String::from("harina"), String::from("leche"), String::from("coca-cola"), String::from("yerba"), String::from("alcohol")];
    // let mut prices: [f64;8] = [ 41.4, 25.56, 66.6, 2.3, 32.48, 9.1, 2.12, 7.45];

    // let names_to_update: [String;3] = [String::from("azucar"), String::from("coca-cola"), String::from("yerba")];
    // let new_price: f64 = 6.8;

    // println!("{:#?}", prices);
    // entregable_1::juan_manuel_vila_v1::actualizar_precios(names, &mut prices, names_to_update, new_price);
    
    // println!("{:#?}", prices);
    
    

    // let mut dato1= 0.453;
    // mi_funcion(& mut dato1);
    // println!("{}", dato1);
    
    // let dato1= "Juan Manuel Vila";
    // let dato2= "Rocio Tamara Fiser Mayora";
    
    // let resultado = mi_funcion(& dato1, & dato2);

    // println!("{}", resultado);


    // let cadena = String::from("Esta es una cadena de ejemplo.");

    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("error");

    // let value: char = input.trim().parse().expect("No es un char");
    
    // println!( "Cantidad de veces que aparece '{}' en la cadena {:?} -> {:?}", value, cadena, cadena.matches(value).collect::<Vec<_>>().len() );


}

// fn mi_funcion(dato: & mut f64){
//     *dato += 0.1;
//     println!("muestro dato en la funcion: {}", *dato);
// }

// fn mi_funcion<'a>( dato1: &'a  str, dato2: &'a  str) -> &'a str{
//     if dato1.len() > dato2.len() {
//       println!("{}",dato1);
//       return dato1;
//     } else {
//         println!("{}",dato2);
//         return dato2;
//     }
// }