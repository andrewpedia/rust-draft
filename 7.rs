use std::mem::transmute;

trait What {
    fn my_name(&self) -> String;
}
#[repr(transparent)]
struct Lol(u64);

impl What for Lol {
    fn my_name(&self) -> String {
        format!("Elichai{}", self.0)
    }
}

#[repr(C)]
struct WhatDyn {
    data: *const (),
    vtable: *const WhatVtable,
}

#[repr(C)]
#[derive(Debug)]
struct WhatVtable {
    drop: fn(*mut ()),
    size: usize,
    align: usize,
    my_name: fn(*const ()) -> String,
}

fn play_with_vtable(w: &dyn What) {
    let whatd: WhatDyn = unsafe { transmute(w) };
    let inner: &u64 = unsafe { &*(whatd.data as *const u64) };
    println!("inner: {}", inner);
    let vtable: &WhatVtable = unsafe { transmute(whatd.vtable) };
    println!("vtable: {:?}", vtable);
}

fn main() {
    let l = Lol(50);
    play_with_vtable(&l);
}

