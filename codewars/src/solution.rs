fn main() {
    // Test 1
    let num = string_to_number("12345");
    assert_eq!(num, 12345);

    // Test 2
    let arr1 = [1, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(find_smallest_int(&arr1), 1);
    let arr2 = [-1,-55,2,32,0];
    assert_eq!(find_smallest_int(&arr2), -55);

    // Test 3
    let dna1 = String::from("GGCCAATTTTTGCTACGATCGATCGCTATCACGATCTACA");
    assert_eq!(dna_to_rna(&dna1) , "GGCCAAUUUUUGCUACGAUCGAUCGCUAUCACGAUCUACA")
}

// Train 1: Convert a String to a Number!
fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

// Train 2: Find the smallest integer in the array
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

// Train 3: DNA to RNA Conversion
fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}