// Train 1: Convert a String to a Number!
pub fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

// Train 2: Find the smallest integer in the array
pub fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

// Train 3: DNA to RNA Conversion
pub fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

// Train 4: 
