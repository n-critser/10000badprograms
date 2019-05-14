
use std::cmp::Ordering;

fn main() {
    let mut x:f64 = 5.0;
    println!("x:{}",x);
    x = 6.0;
    println!("x:{}",x);
    let a = 2.0;
    let b = 2.0;
    let c = 8.0;
    x = (a - b) / c;
    println!("x:{}",x);
    let a = 2;
    let c = 8;
    let mut z = 50;
    loop {
        let res = get_congruence(c,a,z);//(a - z) % c;
        match res.cmp(&res) {
            Ordering::Less => println!("too small: {}", res),
            Ordering::Greater => println!("too big : {}",res),
            Ordering::Equal => {
                println!("goldilocks wins: {}", res);
                //break;
            }
        }
        if z == 0{

            break;
        }
        println!("gcd :c-{} z-{} =  {}",c,z,get_gcd(c,z));
        z = z -1;
        
        
    }
    prove_congruence();
    let gcd= get_gcd(20,10);
    
    println!("gcd: {}", gcd);
}

fn get_congruence(modulus: i32, right: i32, left: i32) -> i32{
    (right - left) % modulus
}

fn prove_congruence(){
    println!("proving congruence now");
}

fn get_gcd(left: i32, right: i32) -> i32{

    let mut max :i32 = if left < right {
        right
    } else {
        left
    };

    let mut min :i32 = if right <=left {
        right
    }else{
        left
    };

    let mut minus :i32;
    let gcd = loop {

        if max  == 0 && min > 0 {
            break min;
        } else if  max > 0 &&  min == 0 {
            break max;
        } else if max % min == 0 {
            break min;
        } else {
            minus = max - min;
            max  = if minus > min {
                minus
            } else {
                min
            };

            min= if min <=minus {
                min
            }else{
                minus
            };

        }
    };
    gcd
}
