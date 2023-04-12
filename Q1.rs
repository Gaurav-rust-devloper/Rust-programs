fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let (mut i, mut j) = (0, s.len() - 1);

    while i < j {
        
        if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
            return false;
        }
        i += 1; 
        j -= 1; 
    }

    true
}

fn main() {
    let s1 = "racecar";
    let s2 = "hello";
    let s3 = "A man a plan a canal Panama";
    
    println!("Is '{}' a palindrome? {}", s1, is_palindrome(s1));
    println!("Is '{}' a palindrome? {}", s2, is_palindrome(s2));
    println!("Is '{}' a palindrome? {}", s3, is_palindrome(s3));
}
 