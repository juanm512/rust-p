// 7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
// enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
// modificando el contenido del arreglo.

const VALOR_FIJO: u64 = 2;

pub(crate) fn run(){

    let mut nums = [13,2,33,94,512,6];

    for i in 0..nums.len() {
        print!("num[{}] * VALOR_FIJO = {} * {} = ", i, nums[i], VALOR_FIJO);
        nums[i] *= VALOR_FIJO; 
        print!("{}\n", nums[i]);

    } 

}