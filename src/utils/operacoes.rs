pub fn soma(a: i32, b: i32, histo: &mut [String; 6], position: &mut i32) -> i32 {
    let result = a + b;

    println!("{} + {} = {}", a, b, result);
    if *position < 6 {
         
        histo[*position as usize] = format!("{} + {} = {}", a, b, result);
        *position += 1;
    }
    result
}

pub fn sub(a: i32, b: i32, histo: &mut [String; 6], position: &mut i32) -> i32 {
    let result = a - b;

    println!("{} + {} = {}", a, b, result);
    if *position < 6 {
        print!("index {}", *position);
        histo[*position as usize] = format!("{} - {} = {}", a, b, result);
        *position += 1;
    }
    result
}

pub fn mult(a: i32, b: i32, histo: &mut [String; 6], position: &mut i32) -> i32 {
    let result = a - b;

    println!("{} + {} = {}", a, b, result);
    if *position < 6 {
        print!("index {}", *position);
        histo[*position as usize] = format!("{} - {} = {}", a, b, result);
        *position += 1;
    }
    result
}

pub fn divi(a: i32, b: i32, histo: &mut [String; 6], position: &mut i32) -> i32 {
    let result = a - b;

    println!("{} + {} = {}", a, b, result);
    if *position < 6 {
        print!("index {}", *position);
        histo[*position as usize] = format!("{} - {} = {}", a, b, result);
        *position += 1;
    }
    result
}