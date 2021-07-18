fn main() {
    let a = 3;
    let mut b = 4;
    let a1 = &a;
    let a2 = &mut b;
    let a3 = a1 as *const i32;
    let a4 = a2 as *mut i32;
    unsafe {
        println!("a3 {}", *a3);
        println!("a4 {}", *a4);
    }
}

