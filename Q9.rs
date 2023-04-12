fn reverse_string(s: &str) -> String {

    let mut chars: Vec<char> = s.chars().collect();  
    let mut left = 0;
    let mut right = chars.len() - 1;
    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    let reversed_str: String = chars.into_iter().collect();

    reversed_str
}

fn main() {
    let input = "Hello, World!";
    let reversed = reverse_string(input);
    println!("Input: {}", input);
    println!("Reversed: {}", reversed);
}
