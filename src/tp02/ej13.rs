// 13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
// ordena en orden alfabético

pub fn run(){
    let mut cadenas = [
        String::from("the three bodys problem"), 
        String::from("juan"), 
        String::from("chernobyl"),
        String::from("hola"),
        String::from("abecedario"),
        String::from("mundo"),
        String::from("rust"),
        String::from("hilos")
    ];

    println!("{:?}", cadenas);
    ordenar_nombres(&mut cadenas);
    println!("ORDENADO: {:?}", cadenas);
   
}

fn ordenar_nombres( cadenas: &mut [String; 8]) {
    cadenas.sort()
}