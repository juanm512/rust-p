pub fn run(){
    let num: i32 = 15;
    println!("El número {} es par? -> {:?}", num, es_par(num))
}

pub fn es_par(num: i32) -> bool{
    if num % 2 == 0 {
        return true;
    }else{
        return false;
    }
}

#[test]
fn test_es_par(){
    let num1: i32 = 15;
    let num2: i32 = 24;

    assert!(!es_par(num1));
    assert!(es_par(num2));
}