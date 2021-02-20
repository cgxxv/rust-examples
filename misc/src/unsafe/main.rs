fn main() {
    raw_pointer();

    unsafe {
        dangerous();
    }

    using_unsafe_method_safely();

    extern_invoking();

    modify_mut_static_variable(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    unsafe_trait();
}

fn unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

static mut COUNTER: u32 = 0;

fn modify_mut_static_variable(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn extern_invoking() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

fn using_unsafe_method_safely() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    println!("{:?}", a);
    println!("{:?}", b);

    let (a,b) = r.split_at_mut(3);
    println!("{:?}", a);
    println!("{:?}", b);

    //assert_eq!(a, &mut [1, 2, 3]);
    //assert_eq!(b, &mut [4, 5, 6]);

}

//bad sample, this code will lead to undefined action
#[allow(unused)]
fn bad_sample() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}

unsafe fn dangerous() {}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}
