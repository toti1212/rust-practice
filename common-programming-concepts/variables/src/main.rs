const MAX_NUM: u32 = 100_000;

fn main() {
    // ==== Variables and Mutability ====
    let x = 5;
    println!("the number is {}", x); // 5
    // x = 6; Error: x is not mutable

    let mut y = 5;
    println!("y is {}", y); // 5
    y = 6;
    println!("now, y is {}", y); // 6

    // const vs variables
    println!("the constant is: {}", MAX_NUM);

    // shadowing variables
    let x = x + 1;
    println!("x just existed, and now is: {}", x);

    // if we use mut, we can't change the variable data type
    // let mut spaces = "    ";  // 4 spaces
    // spaces = spaces.len();  // expected `&str`, found `usize`
    // but using shadowing we can;
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces {}", spaces) // 4
    // ============================================================

}
