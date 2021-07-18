fn main(){
    let mut b = Box::new(5);
    *b=3;
    let mut x=5;
    let mut y=Box::new( & mut x);
    let yy:&mut i32=*y;
    *yy=30;
    println!("y is {}",y);
    println!("b is {}",b);

}
