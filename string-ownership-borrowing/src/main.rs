fn main() {
    let s = String::from("book");
    let pl = pluralize(&s);
    println!(
        "I have one {}, you have two {}",
        s,
        // pluralize(s.clone()))
        pl)
}

// clone
// fn pluralize(singular: String) -> String {
//     singular + "s"
// }

fn pluralize(singular: &str) -> String {
    singular.to_owned() + "s"
}



