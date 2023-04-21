use std::mem::align_of;
use std::mem::size_of;

macro_rules! dbg_size {
    ($t: ty) => {
        println!(
            "{}: size {} bytes, align: {} bytes",
            stringify!($t),
            size_of::<$t>(),
            align_of::<$t>()
        );
    };
}

enum Foo {
    A,
    B,
}

fn main() {
    dbg_size!(Foo);
}
