use std::fmt::Display;

pub struct RcBox<T: ?Sized> {
    pub ref_count: usize,
    pub value: T,
}

pub fn display(boxed: &RcBox<Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}
