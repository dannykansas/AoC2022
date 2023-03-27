use std::fs;

fn main() {
    // Read input file into a string
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Split the string into groups of lines (representing each Elf's inventory)
    let groups: Vec<&str> = input.trim().split("\n\n").collect();

    // Find the Elf with the highest total Calories
    let mut max_calories = 0;
    for group in groups {
        let calories: Vec<u32> = group.lines().map(|s| s.parse().unwrap()).collect();
        let total_calories = calories.iter().sum();
        if total_calories > max_calories {
            max_calories = total_calories;
        }
    }

    // Print the result
    println!("The Elf carrying the most Calories is carrying {} Calories.", max_calories);
}

