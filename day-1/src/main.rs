use std::iter::zip;
use std::path::PathBuf;

use utils::read_lines;

fn main() {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("puzzle-input.txt");

    if let Ok(lines) = read_lines(file_path) {
        let mut left_numbers = Vec::new();
        let mut right_numbers = Vec::new();

        for line in lines.map_while(Result::ok) {
            let split_lines: Vec<&str> = line.split("   ").collect();

            assert!(
                split_lines.len() == 2,
                "Line split does not contain two numbers"
            );

            left_numbers.push(
                split_lines[0]
                    .parse::<u32>()
                    .expect("Left number invalid u8"),
            );

            right_numbers.push(
                split_lines[1]
                    .parse::<u32>()
                    .expect("Right number invalid u8"),
            );
        }

        left_numbers.sort();
        right_numbers.sort();

        let numbers_zipped = zip(left_numbers, right_numbers);
        let res: u32 = numbers_zipped
            .map(|(left_number, right_number)| {
                let left = left_number.max(right_number);
                let right = left_number.min(right_number);

                left - right
            })
            .sum();

        println!("{res}");
    }
}
