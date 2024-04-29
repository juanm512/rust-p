// juan manuel, vila, 20572/7, juanm512
pub fn actualizar_precios( nombres: [String;8],  precios: &mut [f64;8], productos_a_actualizar: [String;3], new_precios: f64 ){

    for i in 0..nombres.len() {
        for j in 0..productos_a_actualizar.len() {
            if productos_a_actualizar[j] == nombres[i] {
                precios[i] = new_precios;
            }
        }
    }

}

#[test]
fn test_actualizar_precios(){
    let names: [String;8] = [ String::from("azucar"), String::from("arroz"), String::from("fideos"), String::from("harina"), String::from("leche"), String::from("coca-cola"), String::from("yerba"), String::from("alcohol")];
    let mut prices: [f64;8] = [ 41.4, 25.56, 66.6, 2.3, 32.48, 9.1, 2.12, 7.45];

    let names_to_update: [String;3] = [String::from("azucar"), String::from("coca-cola"), String::from("yerba")];
    let new_price: f64 = 6.8;

    actualizar_precios(names, &mut prices, names_to_update, new_price);

    assert_ne!(41.4, prices[0]);//0 azucar - debe cambiar respecto al valor original

    assert_eq!(2.3, prices[3]);//3 harina - no debe tener precio cambiado

    assert_ne!(9.1, prices[5]);//5 coca-cola - debe cambiar respecto al valor original

    assert_eq!(32.48, prices[4]);//4 leche - no debe cambiar el precio original

}