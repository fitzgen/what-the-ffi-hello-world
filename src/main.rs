extern crate libc;

extern "C" {
    fn hello_world();

    fn hello_to(who: *const libc::c_char, times: libc::size_t);

    fn super_fast_add(a: libc::size_t, b: libc::size_t) -> libc::size_t;
}

fn main() {
    unsafe {
        hello_world();

        let who = b"PDXRust\0";
        hello_to(who.as_ptr() as *const _, 3);

        let sum = super_fast_add(4, 38);
        println!("sum of 4 and 38 is {}", sum);
    }
}
