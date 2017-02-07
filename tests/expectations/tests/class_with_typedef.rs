/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub type AnotherInt = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub c: C_MyInt,
    pub ptr: *mut C_MyInt,
    pub arr: [C_MyInt; 10usize],
    pub d: AnotherInt,
    pub other_ptr: *mut AnotherInt,
}
pub type C_MyInt = ::std::os::raw::c_int;
pub type C_Lookup = *const ::std::os::raw::c_char;
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 72usize);
    assert_eq! (::std::mem::align_of::<C>() , 8usize);
    assert_eq! (unsafe { & ( * ( 0 as * const C ) ) . c as * const _ as usize
                } , 0usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . ptr as * const _ as usize } ,
                8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . arr as * const _ as usize } ,
                16usize);
    assert_eq! (unsafe { & ( * ( 0 as * const C ) ) . d as * const _ as usize
                } , 56usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . other_ptr as * const _ as usize }
                , 64usize);
}
extern "C" {
    #[link_name = "_ZN1C6methodEi"]
    pub fn C_method(this: *mut C, c: C_MyInt);
}
extern "C" {
    #[link_name = "_ZN1C9methodRefERi"]
    pub fn C_methodRef(this: *mut C, c: *mut C_MyInt);
}
extern "C" {
    #[link_name = "_ZN1C16complexMethodRefERPKc"]
    pub fn C_complexMethodRef(this: *mut C, c: *mut C_Lookup);
}
extern "C" {
    #[link_name = "_ZN1C13anotherMethodEi"]
    pub fn C_anotherMethod(this: *mut C, c: AnotherInt);
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
impl C {
    #[inline]
    pub unsafe fn method(&mut self, c: C_MyInt) { C_method(&mut *self, c) }
    #[inline]
    pub unsafe fn methodRef(&mut self, c: *mut C_MyInt) {
        C_methodRef(&mut *self, c)
    }
    #[inline]
    pub unsafe fn complexMethodRef(&mut self, c: *mut C_Lookup) {
        C_complexMethodRef(&mut *self, c)
    }
    #[inline]
    pub unsafe fn anotherMethod(&mut self, c: AnotherInt) {
        C_anotherMethod(&mut *self, c)
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct D {
    pub _base: C,
    pub ptr: *mut C_MyInt,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(::std::mem::size_of::<D>() , 80usize);
    assert_eq! (::std::mem::align_of::<D>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const D ) ) . ptr as * const _ as usize } ,
                72usize);
}
impl Clone for D {
    fn clone(&self) -> Self { *self }
}
