fn main(){
    // Test 1
    let num = string_to_number("12345");
    assert_eq!(num , 12345);
}

// Train 1: Convert a String to a Number!
fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

