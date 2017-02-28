extern "C" {
    fn hello_world();
}

fn main() {
    unsafe {
        hello_world();
    }
}
