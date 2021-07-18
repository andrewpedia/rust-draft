
#[derive(Debug)]
struct A<'_a> {
    data: i32,
    num: i32,
    count:i32,
    name:&'_a str,
}

struct B<'_a> {
    a: *mut A<'_a>,
}

impl<'_a> Drop for  A<'_a>{
    fn drop(&mut self) {
        println!("Dropping {}!",self.name);
    }
}

impl<'_a> A<'_a>{

    pub fn new() -> A<'_a>{
	let mut aa = A{
name:"",data:10,
num:12,count:22,};
	
aa.name="aa";
aa.num+=1;

        let mut a = Box::new(A{
            data: 0,
	    num:1,
            name:"",
	    count:100,
        });
	
a.num+=1;
a.name="a";
a.count+=2;

	A{
	data:0,num:10,count:1000,name:"A",
}
    }


}

impl<'_a> B<'_a>{

    fn set_data(&self, data: i32) {
        unsafe { (*self.a).data = data };
    }
}

fn main(){
    let a = A::new();
    println!("{}",a.num);
    println!("{}","start".to_string());
    let string1 = String::from("golang");
    let maybe_string = Some(String::from("foo"));
    let ret = maybe_string.as_ref().map(String::as_str).unwrap_or("none");
    
    println!("ret :{}",ret);
	
    let mut x = Some(2);
    match x.as_mut() {
    	Some(v) => *v = 42,
    	None => {},
    }
    
    assert_eq!(x, Some(42));


}
