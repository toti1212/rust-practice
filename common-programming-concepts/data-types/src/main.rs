use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // ==================== Data Types ====================
    // Integers ========================
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
    let f2 = 1.0; //f64 by default
    let f1: f32 = 1.9;
    // another way
    let f3 = 1.57_f32;
    println!(
        "this is a float: {} and a negegative float: {}, and the last one: {}",
        f1, f2, f3
    );

    println!("floor: {}, ceil: {}, round: {} ", f1.floor(), f1.ceil(), f3.round());

    // operations

    // additions
    let _n1 = 9_i8;
    let _f1 = 1_f32;
    let _f2 = 1_f64;
    let _f3 = _f1;
    // let sums = n1 + f1;  // I can't becaouse types: int + float
    // let sums = f1 + f2;  // same between floats
    let sums = _f1 + _f3;
    println!("additon: f1 + f3 = {}",sums);

    // substraction
    let subs = 5.0 - 0.1;
    println!("substraction: {}", subs);

    // multiplication
    let mult = 5.0 * 1.1;
    println!("mult: {}", mult);

    // division
    let division = 2 / 3;  // integers
    println!("division ints: {}", division);

    let division = 2_f32 / 3_f32;  // integers
    println!("division floats: {}", division);

    // reminder
    let rem = 10 % 3;
    println!("reminder: {}", rem);

    // ... continue
    // ============================================================
}
