
fn is_congruent(left: i128, right:i128, modulus:i128) -> bool{

    (left - right) % modulus == 0
}

#[test]
fn test_is_congruent() {
    assert_eq!(is_congruent(3, 0,3), true);

}



fn main() {
    println!("congruent - {}", is_congruent(3,0,3));

    let heart_eyed_cat = '😻';
    println!("cat: {}",heart_eyed_cat);
    // let mut tup_abc:(i128,i128,i128) =(2,3,5);
    // tup_abc = (3,3,3);
    // println!("tup_abc: {}",tup_abc);
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}",s);
}


// fn find_congruence(left: i128, right:i128, modulus:i128) ->f64{


//     (left - right) / modulus
// }
