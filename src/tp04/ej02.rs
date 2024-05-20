use std::{cmp::Ordering, collections::LinkedList};

// 2- Dado el siguiente struct:
#[derive(Debug, Clone, PartialEq)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

// a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
// salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
pub fn listado_por_salario(personas: Vec<Persona>, salario_referencia: f64) -> LinkedList<Persona> {
    let mut lista = LinkedList::new();

    personas
        .into_iter()
        .filter(|p| p.salario > salario_referencia)
        .for_each(|p| lista.push_back(p.clone()));

    lista
}

// b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
// y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
// ciudad.
pub fn listado_por_edad_ciudad<'a>(
    personas: Vec<Persona<'a>>,
    edad_ref: u8,
    ciudad_ref: &'a str,
) -> LinkedList<Persona<'a>> {
    let mut lista = LinkedList::new();

    personas
        .into_iter()
        .filter(|p| p.edad > edad_ref && p.ciudad == ciudad_ref)
        .for_each(|p| lista.push_back(p.clone()));

    lista
}

// c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
// retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
// contrario.
pub fn todas_personas_misma_ciudad<'a>(personas: Vec<Persona<'a>>, ciudad_ref: &'a str) -> bool {
    personas.into_iter().all(|p| p.ciudad == ciudad_ref)
}

// d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
// retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
// contrario.
pub fn almenosuna_personas_en_ciudad<'a>(personas: Vec<Persona<'a>>, ciudad_ref: &'a str) -> bool {
    personas.into_iter().any(|p| p.ciudad == ciudad_ref)
}

// e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
// persona existe en el arreglo, false caso contrario
pub fn existe_persona<'a>(personas: Vec<Persona<'a>>, persona: Persona) -> bool {
    personas.into_iter().find(|p| *p == persona).is_some()
}

// f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
// edades de las personas.
pub fn arreglo_edades<'a>(personas: Vec<Persona<'a>>) -> Vec<u8> {
    personas.into_iter().map(|p| p.edad.clone()).collect()
}

// g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
// salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
// categoría desempatar por la edad más grande.
fn compare_salarios(x_salario: f64, y_salario: f64) -> Ordering {
    if x_salario == y_salario {
        Ordering::Equal
    } else if x_salario > y_salario {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
pub fn personas_min_max_salario<'a>(personas: Vec<Persona<'a>>) -> (Persona, Persona) {
    let personas_clone = personas.clone();

    (
        personas
            .into_iter()
            .min_by(|x, y| compare_salarios(x.salario, y.salario))
            .unwrap(),
        personas_clone
            .into_iter()
            .max_by(|x, y| compare_salarios(x.salario, y.salario))
            .unwrap(),
    )
}

// Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
// Todos los ejercicios deben resolverse con iterator y closure.

pub fn run() {
    let personas = vec![
        Persona {
            nombre: "Juan",
            apellido: "Pérez",
            direccion: "Calle 1",
            ciudad: "Ciudad 1",
            salario: 5100.0,
            edad: 30,
        },
        Persona {
            nombre: "María",
            apellido: "González",
            direccion: "Calle 2",
            ciudad: "Ciudad 2",
            salario: 6000.0,
            edad: 25,
        },
        Persona {
            nombre: "Pedro",
            apellido: "Rodríguez",
            direccion: "Calle 3",
            ciudad: "Ciudad 1",
            salario: 4000.0,
            edad: 35,
        },
    ];

    let salario_referencia = 5000.0;

    let listado_salarios = listado_por_salario(personas.clone(), salario_referencia);
    println!();
    println!();
    println!("------------------ A) -----------------");
    for p in listado_salarios {
        println!("{:?}", p);
    }

    let listado_edad_ciudad = listado_por_edad_ciudad(personas.clone(), 27, "Ciudad 1");
    println!();
    println!();
    println!("------------------ B) -----------------");
    for p in listado_edad_ciudad {
        println!("{:?}", p);
    }

    let todas_personas_misma_ciudad = todas_personas_misma_ciudad(personas.clone(), "Ciudad 1");
    println!();
    println!();
    println!("------------------ C) -----------------");
    println!("Son todos misma ciudad: {}", todas_personas_misma_ciudad);

    let almenosuna_personas_en_ciudad = almenosuna_personas_en_ciudad(personas.clone(), "Ciudad 1");
    println!();
    println!();
    println!("------------------ D) -----------------");
    println!("Hay al menos uno: {}", almenosuna_personas_en_ciudad);

    let existe_persona = existe_persona(
        personas.clone(),
        Persona {
            nombre: "Pedro",
            apellido: "Rodríguez",
            direccion: "Calle 3",
            ciudad: "Ciudad 1",
            salario: 4000.0,
            edad: 35,
        },
    );
    println!();
    println!();
    println!("------------------ E) -----------------");
    println!("Existe la persona en el vector: {}", existe_persona);

    let vec_edades = arreglo_edades(personas.clone());
    println!();
    println!();
    println!("------------------ F) -----------------");
    println!("Existe la persona en el vector: {:?}", vec_edades);

    let personas_min_max_salario = personas_min_max_salario(personas.clone());
    println!();
    println!();
    println!("------------------ G) -----------------");
    println!(
        "Existe la persona en el vector: {:?}",
        personas_min_max_salario
    );
}
