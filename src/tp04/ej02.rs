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

fn create_personas() -> Vec<Persona<'static>> {
    vec![
        Persona {
            nombre: "Juan",
            apellido: "Perez",
            direccion: "Calle 123",
            ciudad: "Ciudad A",
            salario: 3000.0,
            edad: 30,
        },
        Persona {
            nombre: "Maria",
            apellido: "Gomez",
            direccion: "Avenida 456",
            ciudad: "Ciudad B",
            salario: 4000.0,
            edad: 25,
        },
        Persona {
            nombre: "Pedro",
            apellido: "Lopez",
            direccion: "Boulevard 789",
            ciudad: "Ciudad A",
            salario: 5000.0,
            edad: 35,
        },
        Persona {
            nombre: "Ana",
            apellido: "Martinez",
            direccion: "Plaza 101",
            ciudad: "Ciudad C",
            salario: 2000.0,
            edad: 28,
        },
    ]
}

#[test]
fn test_listado_por_salario() {
    let personas = create_personas();
    let resultado = listado_por_salario(personas.clone(), 3500.0);

    let mut expected = LinkedList::new();
    expected.push_back(personas[1].clone());
    expected.push_back(personas[2].clone());

    assert_eq!(resultado, expected);
}

#[test]
fn test_listado_por_edad_ciudad() {
    let personas = create_personas();
    let resultado = listado_por_edad_ciudad(personas.clone(), 28, "Ciudad A");

    let mut expected = LinkedList::new();
    expected.push_back(personas[0].clone());
    expected.push_back(personas[2].clone());

    assert_eq!(resultado, expected);
}

#[test]
fn test_todas_personas_misma_ciudad_true() {
    let personas = vec![
        Persona {
            nombre: "Juan",
            apellido: "Perez",
            direccion: "Calle 123",
            ciudad: "Ciudad A",
            salario: 3000.0,
            edad: 30,
        },
        Persona {
            nombre: "Pedro",
            apellido: "Lopez",
            direccion: "Boulevard 789",
            ciudad: "Ciudad A",
            salario: 5000.0,
            edad: 35,
        },
    ];
    let resultado = todas_personas_misma_ciudad(personas, "Ciudad A");
    assert!(resultado);
}

#[test]
fn test_todas_personas_misma_ciudad_false() {
    let personas = create_personas();
    let resultado = todas_personas_misma_ciudad(personas, "Ciudad A");
    assert!(!resultado);
}

#[test]
fn test_almenosuna_personas_en_ciudad_true() {
    let personas = create_personas();
    let resultado = almenosuna_personas_en_ciudad(personas, "Ciudad B");
    assert!(resultado);
}

#[test]
fn test_almenosuna_personas_en_ciudad_false() {
    let personas = create_personas();
    let resultado = almenosuna_personas_en_ciudad(personas, "Ciudad D");
    assert!(!resultado);
}

#[test]
fn test_existe_persona_true() {
    let personas = create_personas();
    let persona = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Calle 123",
        ciudad: "Ciudad A",
        salario: 3000.0,
        edad: 30,
    };
    let resultado = existe_persona(personas, persona);
    assert!(resultado);
}

#[test]
fn test_existe_persona_false() {
    let personas = create_personas();
    let persona = Persona {
        nombre: "Luis",
        apellido: "Garcia",
        direccion: "Calle 999",
        ciudad: "Ciudad X",
        salario: 6000.0,
        edad: 40,
    };
    let resultado = existe_persona(personas, persona);
    assert!(!resultado);
}

#[test]
fn test_arreglo_edades() {
    let personas = create_personas();
    let resultado = arreglo_edades(personas.clone());
    let expected = vec![30, 25, 35, 28];
    assert_eq!(resultado, expected);
}

#[test]
fn test_personas_min_max_salario() {
    let personas = create_personas();
    let (min_persona, max_persona) = personas_min_max_salario(personas.clone());

    let expected_min = personas[3].clone(); // Ana Martinez with 2000.0 salary
    let expected_max = personas[2].clone(); // Pedro Lopez with 5000.0 salary

    assert_eq!(min_persona, expected_min);
    assert_eq!(max_persona, expected_max);
}
