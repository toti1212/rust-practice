use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // ==================== Data Types ====================
    // Integers ========================
    println!(" === Integers ===");

    let _n1: u8 = 255;
    // let n1_overflow: u8 = 256;
    let _n2: i8 = 127;
    let _n2: i8 = -128;

    let mut number_overflow: u8 = 0;
    for _ in 0..2 {
        // <-- warning
        // if I change this to 256 I will couse an overflow in debug,
        // 0 in production (0.0)!
        number_overflow += 1;
        println!("my future number overflow is: {}", number_overflow);
    }

    // isizes and usizes
    let depends: isize = 2_147_483_648;
    println!(
        "My type I will depends on the machine I'll run {}",
        type_of(&depends)
    );

    // inline types
    let hexas = 0xff;
    let octals = 0o77;
    let bytes = b'R';

    println!("hexas: {}, octas: {}, bytes: {}", hexas, octals, bytes); // 255, 63, 82

    // Floats ========================
    println!("\n=== Floats ===");

    let f2 = 1.0; //f64 by default
    let f1: f32 = 1.9;
    // another way
    let f3 = 1.57_f32;
    println!(
        "this is a float: {} and a negegative float: {}, and the last one: {}",
        f1, f2, f3
    );

    println!(
        "floor: {}, ceil: {}, round: {} ",
        f1.floor(),
        f1.ceil(),
        f3.round()
    );

    // operations

    // additions
    let _n1 = 9_i8;
    let _f1 = 1_f32;
    let _f2 = 1_f64;
    let _f3 = _f1;
    // let sums = n1 + f1;  // I can't becaouse types: int + float
    // let sums = f1 + f2;  // same between floats
    let sums = _f1 + _f3;
    println!("additon: f1 + f3 = {}", sums);

    // substraction
    let subs = 5.0 - 0.1;
    println!("substraction: {}", subs);

    // multiplication
    let mult = 5.0 * 1.1;
    println!("mult: {}", mult);

    // division
    let division = 2 / 3; // integers
    println!("division ints: {}", division);

    let division = 2_f32 / 3_f32; // integers
    println!("division floats: {}", division);

    // reminder
    let rem = 10 % 3;
    println!("reminder: {}", rem);

    // Booleans ========================
    println!("\n=== Booleans ===");

    let learn_rust = true;
    let is_full_stack: bool = true;

    if is_full_stack && learn_rust {
        println!("Learning Rust! ")
    }

    // Characters ========================
    println!("\n=== Characters ===");
    // Chars use 4 bytes
    let letter = 'L';
    let emoji = '🦀';
    println!(
        "This characters must use single quotes -> {} -> {}",
        letter, emoji
    );

    // Tuples ========================
    println!("\n=== Tuples ===");

    let tup: (i32, i32, f32) = (0, 0, 1.1);
    // We can't print a tupple without destructuring it.
    // print!("tupple example {}", tup);

    let (x, y, z) = tup;
    println!("This tuple has x={} y={} z={}", x, y, z);
    println!("Also, we can access by this way: tup.0={} tup.1={} tup.2={}", tup.0, tup.1, tup.2);

    // Arrays ========================
    println!("\n=== Arrays ===");

    let names = ["Rodrigo", "Jhon"];
    let lastnames: [&str; 2] = ["Suarez", "Jhon"];
    let first_name = names[0];
    let first_lastname = lastnames[0];

    println!("firstname {} and lastname {}", first_name, first_lastname);

    // acccessing last element using shadowing
    let last_element = names.len();
    let last_element = names[last_element - 1];
    println!("The last name is {}", last_element);

    // invalid index
    //println!("{}", names[5]);  //index out of bounds: the len is 2 but the index is 5

    let same_numbers = [1; 5];
    for x in &same_numbers {
        println!("is the same number => {}", x);
    }
    // ============================================================
}
