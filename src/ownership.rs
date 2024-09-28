// src/ownership.rs

pub fn demonstrate_ownership() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); 
    let s3=vacation(s2.clone());
    println!("{}",s2);
    println!("{}", s1);
    println!("{}",s3); 
}


fn vacation(some_string:String)->String{
    println!("{}",some_string);
    return some_string;

}