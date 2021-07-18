struct A{
    a:i8
}

trait t{
    fn dosomething(&self);
}

impl t for A{
    fn dosomething(&self){
        println!("{}",self.a);
    }
}

fn main(){

    let a = String::from("hello kitty");
    println!("{}",a);

    let b:A=A{a:12};
    b.dosomething();
    


}
