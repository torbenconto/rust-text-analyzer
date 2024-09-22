use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::env;

fn analyze_text(file_content: &str) -> Result<(usize, Vec<(char, usize, f64)>), Box<dyn Error>> {
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

    let mut sorted_counts: Vec<(char, usize, f64)> = char_count_map.iter()
        .map(|(&ch, &count)| (ch, count, (count as f64 / char_count as f64) * 100.0))
        .collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    Ok((char_count, sorted_counts))
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    
    if argv.len() < 2 {
        println!("usage: rust-text-analyzer <file_path>");
        return Err("Insufficient arguments".into());
    }

    let file_path: &String = &argv[1];

    // Read from file
    let file_content: String = fs::read_to_string(file_path)?;
    
    let (char_count, sorted_counts) = analyze_text(&file_content)?;

    println!("character count: {} (no whitespace)", char_count);
    for (ch, count, percentage) in sorted_counts {
        println!("'{}': {} | {:.2}%", ch, count, percentage);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text() {
        let content = "Hello, world!";
        let (char_count, sorted_counts) = analyze_text(content).unwrap();

        assert_eq!(char_count, 12);
        assert_eq!(sorted_counts.len(), 9);
        assert_eq!(sorted_counts[0].0, 'l');
        assert_eq!(sorted_counts[0].1, 3);
        assert_eq!(sorted_counts[0].2, 25.0);
        assert_eq!(sorted_counts[1].0, 'o');
        assert_eq!(sorted_counts[1].1, 2);
        assert_eq!(sorted_counts[1].2, 16.666666666666664);
    }

    #[test]
    fn test_analyze_text_empty() {
        let content = "";
        let (char_count, sorted_counts) = analyze_text(content).unwrap();

        assert_eq!(char_count, 0);
        assert_eq!(sorted_counts.len(), 0);
    }

    #[test]
    fn test_analyze_text_with_whitespace() {
        let content = "  Test  ";
        let (char_count, sorted_counts) = analyze_text(content).unwrap();

        assert_eq!(char_count, 4);
        assert_eq!(sorted_counts.len(), 3);
        assert_eq!(sorted_counts[0].0, 't');
        assert_eq!(sorted_counts[0].1, 2);
        assert_eq!(sorted_counts[0].2, 50.0);
    }
}