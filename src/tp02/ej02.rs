

pub fn run(){
    let num = 13;
    println!("Es primo el {} -> {:?}", num, es_primo(num) );
}

fn es_primo( num: i32 ) -> bool{
    if num <= 1 {
        return false; 
    }
    
    for i in 2..num { 
        if num % i == 0 {
            return false;
        } 
    }

    return true;
}


#[test]
pub fn test_es_primo(){ 
    assert!( es_primo(5) ) ;
    assert!( !es_primo(24) ) ;
 }