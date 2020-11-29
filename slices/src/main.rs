fn main() {
    let _array = [1, 2, 3];
    let _vec = vec![1, 2, 3];
    let slice = &_vec[0..2];

    let a_string = "Hello".to_string();
    let slice_string = &a_string[..];

    only_arrays(&_array);
    only_vectors(&_vec);
    // only_strings(&slice_string); this is an error
    
    arrays_and_vectors(&_array);
    arrays_and_vectors(&_vec);
    arrays_and_vectors(&slice);
    
}

fn only_arrays(param: &[i32; 3]) {
    println!("params is {:?}", param);
}

fn only_vectors(param: &Vec<i32>) {
    println!("params is {:?}", param);
}

fn only_strings(param: &[String]) {
    println!("params is {:?}", param);
}

fn arrays_and_vectors(param: &[i32]) {
    println!("params is {:?}", param);
}


