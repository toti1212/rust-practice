fn test_variables() {
    println!("Hello, cargo!");
    // Simple built-it types ------
    let x: i32 = 5;  // i32 by default
    let f: f64 = 4.4;  // f64 by default
    let y = 6;
    let z = x + y;
    // let mut signed_value: u8 = -1; // u-> solo positivo
    let mut signed_value: i8 = -1;
    // bool
    let mut b = false;
    // claro, xq no ponerlo en true de una no?
    b = true;
    println!("{}", b);
    // isize or usize
    let v = [1,2,3,4];
    // char
    let c = 'a';

    // Compound built-it types
    // Tuples
    let ituple = (1, 'a', false);
    println!("{}", ituple.2);
    let (t1, t2, t3) = ituple;
    // Array (must have the same TYPE)
    let mut arr = [1,2,3, 4];
    let second = arr[2];
    println!("{}", second);
    arr[2] = 100;
    println!("{}", arr[2]);
    println!("{:?}", arr);
    // arr += 1; No se puede agrandar. Usar Vec instead.

    // Slices
    let mut arr2 = [100, 200, 300];
    let s1 = &arr2[0..1];
    println!("{:?}", s1);
    // Whats going on?
    //arr2[0] = 500;
    //println!("{:?}", s1);
}
fn main() {
    // test_variables();
    println!("My name has {} letters", length_name("Rodrigo"));
}

fn length_name(param: &str) -> usize {
    let x = param.len();
    println!("x {}", x);
    param.len()
}
