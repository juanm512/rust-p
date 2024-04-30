// 6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
// nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
// conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
// métodos:
// ❖ Examen:
// ➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
// retorna.
// ❖ Estudiante:
// ➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
// retorna.
// ➢ obtener_promedio: retorna el promedio de las notas.
// ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
// ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
// Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen.

#[derive(Debug)]
struct Examen{
    nombre_materia: String,
    nota: f32,
}
impl Examen {
    pub fn new( 
        nombre_materia: String,
        nota: f32 
    ) -> Examen{
        Examen{ nombre_materia, nota }
    }
}


#[derive(Debug)]
struct Estudiante{
    id: i64,
    nombre: String,
    calificaciones: Box<[Examen]>
}

impl Estudiante {
    pub fn new(
        id: i64,
        nombre: String,
        calificaciones: Box<[Examen]>
    ) -> Estudiante{
        Estudiante{ id, nombre, calificaciones }
    }

    pub fn obtener_promedio( &self ) -> f32{
        let mut suma = 0.0;
        for i in 0..self.calificaciones.len() {
            suma += self.calificaciones[i].nota;
        }
        suma / self.calificaciones.len() as f32
    }

    pub fn obtener_calificacion_mas_alta( &self ) -> f32 {
        let mut nota_mas_alta = 0.0;
        for i in 0..self.calificaciones.len() {
            if self.calificaciones[i].nota > nota_mas_alta {
                nota_mas_alta = self.calificaciones[i].nota
            }
        }
        nota_mas_alta
    }

    pub fn obtener_calificacion_mas_baja( &self ) -> f32 {
        let mut nota_mas_baja = 10.0;
        for i in 0..self.calificaciones.len() {
            if self.calificaciones[i].nota < nota_mas_baja {
                nota_mas_baja = self.calificaciones[i].nota
            }
        }
        nota_mas_baja
    }

}

const MAT_MATH_A: f32 = 7.7;
const PHYSIC_I: f32 = 5.2;
const MAT_MATH_B: f32 = 9.1;
const MAT_MATH_C: f32 = 8.0;
const PHYSIC_II: f32 = 4.5;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_promedio() {
        let examen_1 = Examen::new(String::from("Matematica A"), MAT_MATH_A);
        let examen_2 = Examen::new(String::from("Fisica I"), PHYSIC_I);
        let examen_3 = Examen::new(String::from("Matematica B"), MAT_MATH_B);
        let examen_4 = Examen::new(String::from("Matematica C"), MAT_MATH_C);
        let examen_5 = Examen::new(String::from("Fisica II"), PHYSIC_II);

        let calificaciones = Box::new([examen_1, examen_2, examen_3, examen_4, examen_5]);

        // Since we can't instantiate an Estudiante directly, we will use a temporary value for testing purposes.
        let estudiante_test = Estudiante {
            id: 101,
            nombre: String::from("Test Student"),
            calificaciones,
        };

        assert!((estudiante_test.obtener_promedio() - (MAT_MATH_A + PHYSIC_I + MAT_MATH_B + MAT_MATH_C + PHYSIC_II) / estudiante_test.calificaciones.len() as f32).abs() < f32::EPSILON);
    }

    #[test]
    fn test_calificacion_mas_alta() {
        let examen_1 = Examen::new(String::from("Matematica A"), MAT_MATH_A);
        let examen_2 = Examen::new(String::from("Fisica I"), PHYSIC_I);
        let examen_3 = Examen::new(String::from("Matematica B"), MAT_MATH_B);
        let examen_4 = Examen::new(String::from("Matematica C"), MAT_MATH_C);
        let examen_5 = Examen::new(String::from("Fisica II"), PHYSIC_II);

        let calificaciones = Box::new([examen_1, examen_2, examen_3, examen_4, examen_5]);

        // Since we can't instantiate an Estudiante directly, we will use a temporary value for testing purposes.
        let estudiante_test = Estudiante {
            id: 102,
            nombre: String::from("Test Student"),
            calificaciones,
        };

        assert_eq!(estudiante_test.obtener_calificacion_mas_alta(), MAT_MATH_B);
    }

    #[test]
    fn test_calificacion_mas_baja() {
        let examen_1 = Examen::new(String::from("Matematica A"), MAT_MATH_A);
        let examen_2 = Examen::new(String::from("Fisica I"), PHYSIC_I);
        let examen_3 = Examen::new(String::from("Matematica B"), MAT_MATH_B);
        let examen_4 = Examen::new(String::from("Matematica C"), MAT_MATH_C);
        let examen_5 = Examen::new(String::from("Fisica II"), PHYSIC_II);

        let calificaciones = Box::new([examen_1, examen_2, examen_3, examen_4, examen_5]);

        // Since we can't instantiate an Estudiante directly, we will use a temporary value for testing purposes.
        let estudiante_test = Estudiante {
            id: 102,
            nombre: String::from("Test Student"),
            calificaciones,
        };

        assert_eq!(estudiante_test.obtener_calificacion_mas_baja(), PHYSIC_II);
    }
}