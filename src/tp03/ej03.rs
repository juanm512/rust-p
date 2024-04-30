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
struct Fecha {
    dia: i16,
    mes: i16,
    anio: i16,
}


impl Fecha {
    pub fn new (dia: i16, mes: i16, anio: i16) -> Fecha{
        Fecha{dia, mes, anio}
    }

    fn calcular_dias_mes(&self) -> i16{
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                return 31
            },
            4 | 6 | 9| 11 => {
                return 30
            },
            2 => {
                if self.es_bisiesto() {
                    return 29
                } 
                return 28
            },
            _ => 30
        }
    }

    pub fn es_fecha_valida (&self) -> bool{
        if  self.mes <= 12 && self.dia <= self.calcular_dias_mes() {
            return true;
        }
        return false;
    }

    pub fn es_bisiesto (&self) -> bool{
      self.anio % 4 == 0 && self.anio % 100 != 0
    }

    fn sumar_dias (& mut self, mut dias: i16){
        
        while dias > 0 {
            if self.dia + dias > self.calcular_dias_mes() {
                dias = dias - (self.calcular_dias_mes() - self.dia) - 1;
                self.dia = 1;
                if self.mes == 12 {
                    self.mes = 1;
                    self.anio += 1;
                } else {
                    self.mes += 1;
                }
            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }
    fn restar_dias (& mut self, mut dias: i16){
        while dias > 0 {
            if (self.dia - dias) < 1 {
                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                dias = dias - (self.dia) ;
                self.dia = self.calcular_dias_mes();
            } else {
                self.dia -= dias;
                dias = 0;
            }
        }
    }
    pub fn es_mayor (&self, fecha: Fecha) -> bool{
        if self.anio > fecha.anio {
            return true;
        } else if self.anio == fecha.anio {
            if self.mes > fecha.mes {
                return true;
            } else if self.mes == fecha.mes {
                if self.dia > fecha.dia {
                    return true;
                } else if self.dia == fecha.dia {
                    return true;
                }else {
                    return false;
                }
            }else {
                return false;
            }
        }else {
            return false;
        }
    }
}

pub fn run(){
    let d = 28;
    let m = 5;
    let a = 2002;
    let mut f1 = Fecha::new(d,m, a);

    println!("{:?}", f1 );
    println!("{:?}", f1.es_fecha_valida() );
    println!();

    f1.restar_dias(250);
    println!();
    println!("{:?}", f1 );
    println!("{:?}", f1.es_fecha_valida() );
}

// tests de validacion de fechas. Probado biciesto con febrero 29 en 2024. Y cuando no lo es. Y caso absurdo de 32 de mayo del xxxx
#[test]
fn test_es_fecha_valida_success_1(){
    let d = 28;
    let m = 5;
    let a = 2002;
    let f1 = Fecha::new(d,m, a);

    assert!(f1.es_fecha_valida());
}
#[test]
fn test_es_fecha_valida_success_2(){
    let d = 29;
    let m = 2;
    let a = 2024;
    let f1 = Fecha::new(d,m, a);

    assert!(f1.es_fecha_valida());
}

#[test]
fn test_es_fecha_valida_fail_1(){
    let d = 32;
    let m = 5;
    let a = 2002;
    let f1 = Fecha::new(d,m, a);

    assert!( !f1.es_fecha_valida());
}

#[test]
fn test_es_fecha_valida_fail_2(){
    let d = 30;
    let m = 2;
    let a = 2002;
    let f1 = Fecha::new(d,m, a);

    assert!( !f1.es_fecha_valida());
}


// validaciones para la funcion de sumar dias. Casos de < 30 dias, >30, >100, >365.
#[test]
fn test_sumar_dias_pocos(){
    let d = 2;
    let m = 2;
    let a = 2024;
    let mut f1 = Fecha::new(d,m, a);
    f1.sumar_dias(12);

    let fecha_esperada = Fecha::new( 14, 2, 2024);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_sumar_dias_mas_de_30(){
    let d = 5;
    let m = 5;
    let a = 2024;
    let mut f1 = Fecha::new(d,m, a);
    f1.sumar_dias(53);

    let fecha_esperada = Fecha::new( 27, 6, 2024);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_sumar_dias_mas_de_100(){
    let d = 23;
    let m = 7;
    let a = 2000;
    let mut f1 = Fecha::new(d,m, a);
    f1.sumar_dias(144);

    let fecha_esperada = Fecha::new( 14, 12, 2000);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_sumar_dias_mas_de_365(){
    let d = 14;
    let m = 11;
    let a = 1988;
    let mut f1 = Fecha::new(d,m, a);
    f1.sumar_dias(512);

    let fecha_esperada = Fecha::new( 10, 4, 1990);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}


// validaciones para la funcion de restar dias. Casos de < 30 dias, >30, >100, >365.
#[test]
fn test_restar_dias_pocos(){
    let d = 29;
    let m = 2;
    let a = 2024;
    let mut f1 = Fecha::new(d,m, a);
    f1.restar_dias(12);

    let fecha_esperada = Fecha::new( 17, 2, 2024);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_restar_dias_mas_de_30(){
    let d = 5;
    let m = 5;
    let a = 2024;
    let mut f1 = Fecha::new(d,m, a);
    f1.restar_dias(53);

    let fecha_esperada = Fecha::new( 13, 3, 2024);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_restar_dias_mas_de_100(){
    let d = 23;
    let m = 7;
    let a = 2000;
    let mut f1 = Fecha::new(d,m, a);
    f1.restar_dias(144);

    let fecha_esperada = Fecha::new( 1, 3, 2000);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}
#[test]
fn test_restar_dias_mas_de_365(){
    let d = 14;
    let m = 11;
    let a = 1988;
    let mut f1 = Fecha::new(d,m, a);
    f1.restar_dias(512);

    let fecha_esperada = Fecha::new( 21, 6, 1987);
    
    assert_eq!( fecha_esperada.dia, f1.dia);
    assert_eq!( fecha_esperada.mes, f1.mes);
    assert_eq!( fecha_esperada.anio, f1.anio);
}