use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::path::PathBuf;

use utils::read_lines;

/// Checks if the direction of the provided current_dir changes or is equal.
///
/// Returns false if we do not change direction or are equal in direction.
///
/// Overrides the provided direction with the new direction
///
/// # Examples
/// ```
/// let mut direction = None;
/// let dir_invalid = check_dir_rule(0, 1, &mut direction);
/// ```
fn check_dir_rule(lhs: i8, rhs: i8, current_dir: &mut Option<Ordering>) -> bool {
    let new_dir = lhs.cmp(&rhs);
    if let Some(current_dir) = current_dir {
        if *current_dir != new_dir {
            return true;
        }
    }

    if new_dir == Ordering::Equal {
        return true;
    }

    *current_dir = Some(new_dir);

    false
}

/// Checks if the lhs or rhs exceeds the provided range.
///
/// Returns false if we do not exceed range.
///
/// # Examples
/// ```
/// let exceeds_range = check_exceeds_range_rule(2, 3, -3..=3);
/// ```
fn check_exceeds_range_rule(lhs: i8, rhs: i8, range: &RangeInclusive<i8>) -> bool {
    let diff = lhs - rhs;
    if !range.contains(&diff) {
        return true;
    }

    false
}

/// Checks if any of the rules are broken from a `&[i8]` slice.
///
/// Returns the number of rules broken in the slice.
///
/// # Examples
///
/// ```
/// let v = vec![0, 1, 2, 3];
/// let rules_broken = check_rules(&v);
/// ```
fn check_rules(line_split: &[i8]) -> u32 {
    let mut dir = None;
    let mut rules_broken: u32 = 0;
    let allowable_range = -3..=3;

    // Check if any rules are broken.
    for val_pos in 0..line_split.len() - 1 {
        let dir_rule_broken =
            check_dir_rule(line_split[val_pos], line_split[val_pos + 1], &mut dir);
        let exceeds_range_rule_broken = check_exceeds_range_rule(
            line_split[val_pos],
            line_split[val_pos + 1],
            &allowable_range,
        );

        // If either rule is broken, increment rules broken counter.
        if dir_rule_broken || exceeds_range_rule_broken {
            rules_broken += 1;
        }
    }

    rules_broken
}

fn main() {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("puzzle-input.txt");

    let lines = read_lines(file_path).expect("Failed to read puzzle-input.txt");
    let mut part_1_safe_count: u32 = 0;
    let mut part_2_safe_count: u32 = 0;

    for line in lines.map_while(Result::ok) {
        let line_split: Vec<i8> = line
            .split(' ')
            .map(|chr| chr.parse::<i8>().expect("Failed to parse puzzle-input.txt"))
            .collect();

        let rules_broken = check_rules(&line_split);

        if rules_broken == 0 {
            part_1_safe_count += 1;
            part_2_safe_count += 1;
        } else {
            // Check if we remove one position if we can fix the issue.
            for pos in 0..line_split.len() {
                let mut test_line_split = line_split.clone();
                test_line_split.remove(pos);

                // Removing one position fixed the rule, break the loop.
                if check_rules(&test_line_split) == 0 {
                    part_2_safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1: {part_1_safe_count}");
    println!("Part 2: {part_2_safe_count}");
}
