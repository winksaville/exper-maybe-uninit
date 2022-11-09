use std::{mem::MaybeUninit, ptr::addr_of_mut};

fn main() {

    // Based on: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-a-struct-field-by-field
    #[derive(Debug, PartialEq)]
    struct S {
        five: u32,
        v: u32,
    }

    let s = {
        let mut s_uninit = MaybeUninit::<S>::uninit();
        let s_ptr = s_uninit.as_mut_ptr();

        unsafe {
            addr_of_mut!((*s_ptr).five).write(5);
            addr_of_mut!((*s_ptr).v).write(1005);
            s_uninit.assume_init()
        }
    };
    assert_eq!(s, S { five: 5, v: 1005 });

    println!("s={s:?}");

}
