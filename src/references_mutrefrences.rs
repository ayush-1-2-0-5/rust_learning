

fn update_string(str: &mut String)
{
    str.push_str("world");
}

pub fn showhowrefworks()
{

    let mut str1=String::from("hello");
    //there can be multiple borrowers who can use the refrence value at a time but not change it
    let s2=refrencetest(&str1);
    let s3=refrencetest(&str1);
    update_string(&mut str1);
    let s4=refrencetest(&str1);
    let s5=refrencetest(&str1);
    let s6=refrencetest(&str1);
   println!("{}",str1);
}


fn refrencetest(some_string:&String){
   println!("{}",some_string);
}


