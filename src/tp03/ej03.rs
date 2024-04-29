// 3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
// mes y el año. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
// ➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en
// cuenta los años bisiestos también.
// ➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
// ➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
// ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
// ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
// la fecha pasada por parámetro.


#[derive(Debug)]
enum Mes {
    ENERO, FEBRERO, MARZO, ABRIL, MAYO, JUNIO, JULIO, AGOSTO, SEPTIEMBRE, OBTUBRE, NOVIEMBRE, DICIEMBRE
}

#[derive(Debug)]
enum MesGrupo{
    FEBRERO(Mes),
    CON30(Mes),
    CON31(Mes)
}

#[derive(Debug)]
struct Fecha {
    dia: i16,
    mes: MesGrupo,
    anio: i16,
}


impl Fecha {
    fn new (dia: i16, mes: MesGrupo, anio: i16) -> Fecha{
        Fecha{dia, mes, anio}
    }

    fn es_fecha_valida (&self) -> bool{
        match &self.mes {
            MesGrupo::FEBRERO(_) => {
                if self.dia <= 28 {
                    return true;
                } else if self.dia == 29  {
                    if self.es_bisiesto() { 
                        return true; 
                    } else {
                        return false;
                    }
                } else {
                    self.dia < 30
                }
            },
            MesGrupo::CON31(_) => {
                self.dia <= 31
            },
            MesGrupo::CON30(_) => {
                self.dia <= 30
            }
        }
    }

    fn es_bisiesto (&self) -> bool{
      self.anio % 4 == 0 && self.anio % 100 != 0
    }

    // fn sumar_dias (& mut self, dias: i16){
    //     // aca revisar cuando los dias a sumar son > a 1 mes o 2 meses , etc
    //     if self.dia + dias > 30 {
    //         self.dia = self.dia + dias - 30;
    //         if self.mes + 1 > 12 {
    //             self.mes = 1;
    //             self.anio += 1;
    //         } else {
    //             self.mes += 1;
    //         }
    //     } else {
    //         self.dia += dias;
    //     }
    // }
    // fn restar_dias (&self, dias: i16){

    // }
    // fn es_mayor (&self, fecha: Fecha) -> bool{
    //     if self.anio > fecha.anio {
    //         return true;
    //     } else if self.anio == fecha.anio {
    //         if self.mes > fecha.mes {
    //             return true;
    //         } else if self.mes == fecha.mes {
    //             if self.dia > fecha.dia {
    //                 return true;
    //             } else if self.dia == fecha.dia {
    //                 return true;
    //             }else {
    //                 return false;
    //             }
    //         }else {
    //             return false;
    //         }
    //     }else {
    //         return false;
    //     }
    // }
}

pub fn run(){
    let d = 28;
    let m = MesGrupo::CON30(Mes::ENERO);
    let a = 2002;
    let f1 = Fecha::new(d,m, a);

    println!("{:?}", f1 );
    println!("{:?}", f1.es_fecha_valida() );
    
}



