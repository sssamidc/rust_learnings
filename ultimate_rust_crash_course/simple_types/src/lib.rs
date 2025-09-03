// print_difference function
pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

// print_array function
pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

// The ding function
pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

// the on_off function
pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}
