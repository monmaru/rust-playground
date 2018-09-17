use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    sort_works(&mut table);

    let r = &factorial(6);
    println!("r = {}", r);
    assert_eq!(r + &1009, 1729);

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
    let st = StringTable {
        elements: aside.into_iter().map(|x| x.to_string()).collect(),
    };
    let answer = st.find_by_prefix("34");
    match answer {
        Some(x) => println!("answer = {}", x),
        None => println!("nothing"),
    }
}

// 共有参照
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

// 可変参照
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

#[allow(dead_code)]
static mut STASH: &i32 = &128;

// STASHはプログラム実行全体を生存期間とするのでpの生存期間も合わせる必要がある。
#[allow(dead_code)]
fn f(p: &'static i32) {
    // 可変なstatic変数はスレッド安全ではないため、unsafeブロックの中でしかアクセスできない。
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

#[allow(dead_code)]
struct S<'a> {
    r: &'a i32,
}

// 生存期間をあわせる必要がある
#[allow(dead_code)]
struct T<'a> {
    s: S<'a>,
}

struct S2<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

#[allow(dead_code)]
fn test() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S2 { x: &x, y: &y };
            r = s.x;
            let z = s.y;
            println!("r = {}, z = {}", r, z);
        }
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}
