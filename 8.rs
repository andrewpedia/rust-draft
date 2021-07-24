fn fork() ->String{
    let str1 = String::from("123");
    return str1;
}

fn main(){
    let s:String = fork();
    println!("{}",s);
}
