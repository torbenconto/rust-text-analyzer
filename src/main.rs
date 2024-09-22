use std::env;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    
    if argv.len() < 2 {
        println!("usage: rust-text-analyzer <file_path>");
        return Err("Insufficient arguments".into());
    }

    let file_path: &String = &argv[1];

    // Read from file
    let file_content: String = fs::read_to_string(file_path)?;
    
    let mut char_count = 0;
    let mut char_count_map: HashMap<char, usize> = HashMap::new();

    for character in file_content.chars() {
        if character.is_whitespace() {
            continue;
        }
        let lower_character = character.to_lowercase().next().unwrap();
        *char_count_map.entry(lower_character).or_insert(0) += 1;
        char_count += 1;
    }
    
    let mut sorted_counts: Vec<(&char, &usize)> = char_count_map.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    println!("character count: {} (no whitespace)", char_count);
    for (ch, count) in sorted_counts {
        println!("'{}': {} | {:.2}%", ch, count, (*count as f64 / char_count as f64) * 100.0);
    }

    Ok(())
}