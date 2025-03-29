use std::env::consts;

fn main() {
    let mut s = String::from("Welcome...");
    let lst = [4,5,6,7,8,9];
    let ptr: *const u8 = lst.as_ptr();
    s.push_str("Bro!");
    let p = s.clone();
    println!("Hello, world! {s} {p}");
    // let z = five(19) + 1;
    let d = ex1(5);
    println!("Data from ex1 fun: {d}.");
    ex2(&lst);
    // if z > 20{
    //     println!("Number is big");
    //     // println!("The value of z is {z}.");
    // }else{
    //     println!("Number is small");
    //     // println!("The value of z is {z}.");
    // }
}

// fn five(x: i8) -> i8 {
//     let n = x;
//     5 + n
// }

fn ex1(x: i8) -> i8 {
    let mut n = x;
    
    loop{
        if n >= 10{
            break (n * 2); 
        }
        println!("Data: {n}");
        n += 1
    };
    n
} 

fn ex2(ptr: &const u8){
    for number in (1..7).rev(){
        println!("N: {number}!");
    }
    println!("LIFTOFF!!!");
}
