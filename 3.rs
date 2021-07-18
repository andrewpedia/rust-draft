fn strlen(b:Box<dyn AsRef<str>>)->usize{
   b.as_ref().as_ref().len()
}

fn coercion(){
    let s = String::from("123");
    let ss:&str = &s;
}

fn main(){
    let b :Box<dyn AsRef<str>> = Box::new(String::from("123"));
    println!("{}",strlen(b));


    let a=3;
    let aa=&a;
    unsafe{
    let aaa=aa as * const i32;
    println!("{}",*aaa);
    }

}
