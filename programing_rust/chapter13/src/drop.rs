pub struct Appelation {
    pub name: String,
    pub nicknames: Vec<String>,
}

/*
trait Drop {
    fn drop(&mut self);
}
*/

impl Drop for Appelation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}
