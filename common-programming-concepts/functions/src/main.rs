fn main() {
    println!("Hi, I'm inside the main");
    outside_main();
    println!("The sum is {}", sum_numbers(1, 1));
    println!("The rest is {}", rest_numbers(1.5 as i32, 1)); // bonus: parse float to int with as
    statements_and_expresions();
    if_else_control_flow();
    loop_control_flow();
    while_control_flow();
    iterators_control_flow();
}

fn outside_main() {
    println!("Hi, I'm outside the main")
}

fn sum_numbers(a: i32, b: i32) -> i32 {
    let x = a + b;
    x // the return is implicit
}

fn rest_numbers(a: i32, b: i32) -> i32 {
    a - b
}

fn statements_and_expresions() {
    let x = 5;
    // You can asign a expresions like functions to a variables
    // expressions do noy include semicolons at the end,
    // if you put that, you are changing it into a statement
    let y = {
        let mut z = x;
        for i in 0..4 {
            z += i;
        }
        z
    };
    println!("The value of y is: {}", y);
}

fn if_else_control_flow() {
    // basic if control flow
    let start = 10;
    let cond = 5;
    if start > cond {
        println!("Is bigger than {}", cond);
    } else if start < cond {
        println!("Is smaller than {}", cond);
    } else {
        println!("last else");
    }

    // if with one line asign
    let cond = true;
    let num = if cond { 10 } else { 0 };
    println!("The value of num is: {}", num);
    // but you can't changes the types...
    let num = if cond { 10 } else { 6 };

    // if with evaluation of true
    // if num {  // error: must to be boolean
    //     print!("The number evaluate to true");
    // }
    if num != 0 {
        // error: must to be boolean
        print!("The number is not 0");
    }
}

fn loop_control_flow() {
    let mut x = 5;
    loop {
        if x == 0 {
            break;
        }
        println!("Im in a loop for: {}", x);
        x = x - 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break counter * 10;
            break counter * 10;
        }
    };
    println!("the result in this loop is {}", result);
}

fn while_control_flow() {
    let mut x = 10;
    while x != 0 {
        println!("Similar to loop, in a WHILE x is: {}", x);
        x = x - 1;
    }
}

fn iterators_control_flow() {
    let numbers = [10, 20, 30, 40, 50];
    for num in numbers.iter() {
        println!("index: {}", num);
    }

    // print reverse numbers
    for i in (1..10).rev() {
        print!("{}-",i);
    }
}
