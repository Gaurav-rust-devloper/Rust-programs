fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = String::new();
    let mut index = 0;
    
    loop {
        let mut chars: Vec<char> = strings.iter()
            .map(|s| s.chars().nth(index))
            .collect();
        
        if let Some(first_char) = chars[0] {
            if chars.iter().all(|&c| c == first_char) {
                prefix.push(first_char);
                index += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);
}
 