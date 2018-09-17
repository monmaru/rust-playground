fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // move
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let _t = s.clone();
        let _u = s.clone();
    }

    {
        let mut s = "Govina".to_string();
        let t = s;
        s = "Siddhartha".to_string(); // nothing is dropped here
        println!("s = {}, t = {}", s, t);
    }

    move_vec();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next)
    }
    println!("P(1..10) = {:?}", padovan); // dropped here
}

struct Person {
    name: String,
    birth: i32,
}

fn move_vec() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);
}
