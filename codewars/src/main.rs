mod solution;
use crate::solution::*;
fn main() {
    // Test 1
    let num = string_to_number("12345");
    assert_eq!(num, 12345);

    // Test 2
    let arr1 = [1, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(find_smallest_int(&arr1), 1);
    let arr2 = [-1, -55, 2, 32, 0];
    assert_eq!(find_smallest_int(&arr2), -55);

    // Test 3
    let dna1 = String::from("GGCCAATTTTTGCTACGATCGATCGCTATCACGATCTACA");
    assert_eq!(
        dna_to_rna(&dna1),
        "GGCCAAUUUUUGCUACGAUCGAUCGCUAUCACGAUCUACA"
    )
}
