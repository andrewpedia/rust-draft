impl<T> DeRef for uint32<T>{
    type target=T
    fn DeRef(&self) -> &Self::Target{
        //not work 
    }
}
fn main(){

	use std::cell::RefCell;

	let c = RefCell::new("hello".to_owned());

    *c.borrow_mut() = "bonjour".to_owned();

    assert_eq!(&*c.borrow(), "bonjour");

    let a:uint32=1;
    let b=*a;  //int have not impl DeRef
}
