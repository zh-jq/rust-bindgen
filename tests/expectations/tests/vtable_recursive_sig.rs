/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Derived {
    pub _base: Base,
}
#[test]
fn bindgen_test_layout_Derived() {
    assert_eq!(::std::mem::size_of::<Derived>() , 8usize);
    assert_eq! (::std::mem::align_of::<Derived>() , 8usize);
}
impl Clone for Derived {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
pub struct Base__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Base {
    pub vtable_: *const Base__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_Base() {
    assert_eq!(::std::mem::size_of::<Base>() , 8usize);
    assert_eq! (::std::mem::align_of::<Base>() , 8usize);
}
impl Clone for Base {
    fn clone(&self) -> Self { *self }
}
