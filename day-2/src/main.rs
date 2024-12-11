use std::cmp::Ordering;
use std::path::PathBuf;
use utils::read_lines;

fn main() {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("puzzle-input.txt");

    let lines = read_lines(file_path).expect("Failed to read puzzle-input.txt");
    let mut safe_count = 0;

    for line in lines.map_while(Result::ok) {
        let line_split: Vec<i8> = line
            .split(' ')
            .map(|chr| chr.parse::<i8>().expect("Failed to parse puzzle-input.txt"))
            .collect();
        let mut dir = None;
        let mut breaks_rule = false;
        let min_range = -3;
        let max_range = 3;

        for val_pos in 0..line_split.len() - 1 {
            // Check if any rules are broken.

            // Check if direction changes.
            let new_dir = line_split[val_pos].cmp(&line_split[val_pos + 1]);
            if let Some(dir) = dir {
                if dir != new_dir {
                    breaks_rule = true;
                    break;
                }
            }

            // Assign direction.
            dir = Some(new_dir);

            // Check if difference falls within' valid range.
            let diff = line_split[val_pos] - line_split[val_pos + 1];
            if !(min_range..=max_range).contains(&diff) {
                breaks_rule = true;
                break;
            }
        }

        if !breaks_rule {
            safe_count += 1;
        }
    }

    println!("{safe_count}");
}
