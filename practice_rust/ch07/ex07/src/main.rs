use std::collections::HashMap;
use std::ops::Drop;

// #[derive(Copy, Clone, Debug)]
#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?} ", self);
    }
}

// #[derive(Copy, Clone, Debug)]
#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // value borrowed here after move
    // println!("p1: {:?}", p1);

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
    f1(&p1);
    f2(&mut p1);
    println!("p1: {:?}", p1);

    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= 2;
}

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    match map.get_mut(&key) {
        Some(value) => value.push_str(", world!"),
        None => {
            map.insert(key, Default::default());
        }
    }
}
