// 1- Escribir un programa que defina una estructura Persona que tenga campos para el
// nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
// persona). Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
// ➢ to_string: que retorna un string con los datos de la persona concatenados sobre el
// mensaje ejecutado por ej:
// person.to_string() , donde person es una variable del tipo Persona.
// ➢ obtener_edad: retorna la edad de la persona.
// ➢ actualizar_direccion(nueva_direccion)


#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: i32,
    direccion: Option<String>,
}

impl Persona {
    fn new (nombre: String, edad: i32, direccion: Option<String>) -> Persona{
        Persona{nombre, edad, direccion}
    }

    fn to_string (&self) -> String{
        let mut s = String::from("Nombre: ");
        s.push_str(&self.nombre);
        s.push_str(" | Edad: ");
        s.push_str(&self.edad.to_string());
        s.push_str(" | Direccion: ");
        match &self.direccion {
            Some(valor) => s.push_str(&valor),
            None => s.push_str("no tiene direcc. registrada")
        }
        s
    }

    fn obtener_edad (&self) -> i32{
        self.edad
    }

    fn actualizar_direccion (&mut self, nueva_direccion: Option<String>){
        self.direccion = nueva_direccion;
    }
}

pub fn run(){
    let nombre: String = String::from("Juan Manuel");
    let edad: i32 = 24;
    let direccion: String = String::from("Olavarria 403");

    let p1 = Persona::new( nombre, edad, Some(direccion));
    println!("{:?}", p1);
    println!();
    println!("{}",p1.to_string());
    
    let nombre: String = String::from("Rocio Tamara");
    let edad: i32 = 23;
    let direccion = None;

    let mut p2 = Persona::new( nombre, edad, direccion);
    println!();
    println!();
    println!("{:?}", p2);
    println!();
    println!("Edad: {}", p2.obtener_edad());
    println!();
    p2.actualizar_direccion(Some(String::from("Artigas 187")));
    println!("{}",p2.to_string());
}

// #[test]
// fn test_persona_new_1(){
//     let nombre: String = String::from("Juan Manuel");
//     let edad: i32 = 24;
//     let direccion: String = String::from("Olavarria 403");

//     // assert!(!es_par(num1));
// }

// #[test]
// fn test_persona_new_2(){
//     let nombre: String = String::from("Rocio Tamara");
//     let edad: i32 = 24;
//     let direccion: Option<String> = None;
   
//     // assert!(es_par(num2));
// }