mod drop;
mod sized;

fn main() {
    // drop
    {
        let mut _a = drop::Appelation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };

        println!("before assignment");
        _a = drop::Appelation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };
        println!("at end of block");
    }

    // sized
    {
        let boxed_lunch: sized::RcBox<String> = sized::RcBox {
            ref_count: 1,
            value: "lunch".to_string(),
        };

        sized::display(&boxed_lunch);
    }
}
