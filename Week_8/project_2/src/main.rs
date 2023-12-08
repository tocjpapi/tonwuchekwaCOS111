use std::io;

fn main() {
    // Get the number of candidates
    println!("Enter the number of candidates:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_candidates: usize = input.trim().parse().expect("Invalid input");

    // Get candidates' names and years of experience
    let mut candidates: Vec<(String, u32)> = Vec::new();
    for _ in 0..num_candidates {
        println!("Enter candidate name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Enter years of experience:");
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).expect("Failed to read line");
        let experience: u32 = experience.trim().parse().expect("Invalid input");

        candidates.push((name.trim().to_string(), experience));
    }

    // Find the candidate with the highest years of experience
    let (index, max_experience) = find_max_experience(&candidates);

    // Print the result
    if let Some(index) = index {
        let (name, _) = &candidates[index];
        println!(
            "The candidate with the highest years of experience is {} with {} years.",
            name, max_experience
        );
    } else {
        println!("No candidates found.");
    }
}

// Function to find the index and maximum years of experience
fn find_max_experience(candidates: &Vec<(String, u32)>) -> (Option<usize>, u32) {
    if candidates.is_empty() {
        return (None, 0);
    }

    let (index, max_experience) = candidates.iter().enumerate().fold(
        (None, 0),
        |acc, (i, (_, experience))| {
            if *experience > acc.1 {
                (Some(i), *experience)
            } else {
                acc
            }
        },
    );

    (index, max_experience)
}
