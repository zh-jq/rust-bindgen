#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod outer {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod inner {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone)]
            pub struct Helper {
                pub _address: u8,
            }
            #[allow(clippy::unnecessary_operation, clippy::identity_op)]
            const _: () = {
                ["Size of Helper"][::std::mem::size_of::<Helper>() - 1usize];
                ["Alignment of Helper"][::std::mem::align_of::<Helper>() - 1usize];
            };
        }
    }
}
