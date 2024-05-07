//A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all 
//non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//Given a string s, return true if it is a palindrome, or false otherwise.


fn main() {
    println!("{}",is_palindrome("racecar".to_string()));
}

fn is_palindrome(s: String) -> bool {
    let s_vector: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    let length = s_vector.len();
    if length < 1 {
        return true;
    }
    let mut index = 0;
    while index <= (length / 2) {
        if &s_vector[index] != &s_vector[length - index - 1] {
            return false;
        }
        index += 1;
    }
    true
}

//take string, remove non-alphanumeric chars, set to lowercase and convert to vec, then compare using a for loop where we compare the string[i] to
//string[len - i], returning if we find an inequality. We do this until we get to the middle, which
//is found by: if string is even, we compare only while i <= len / 2. if odd, it is the same,
//because the middle value is only going to be compared to itself. 
