use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let first = "123";
    let second = "456";

    println!("{:#?}", lattice_multiplication(first, second));
}

fn lattice_multiplication<'a>(first: &'a str, second: &'a str) -> u128 {
    let mut product: u128 = 0;

    for (second_index, second_digit) in second.to_digits().iter().rev().enumerate() {
        for (first_index, first_digit) in first.to_digits().iter().rev().enumerate() {
            product += (second_digit * first_digit) as u128 * (10 as u128).pow((second_index + first_index) as u32);
        }
    }

    product
}

trait Lattice {
    fn to_digits(&self) -> Vec<u8>;
}

impl Lattice for &str {
    fn to_digits(&self) -> Vec<u8> {
        self.graphemes(false)
            .map(|digit| digit.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    }
}
