//Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

fn main() {
    println!("{}",is_anagram("cats".to_string(), "tars".to_string()));
}

fn is_anagram(s: String, t: String) -> bool {
    let mut s_vector: Vec<char> = s.chars().collect();
    let mut t_vector: Vec<char> = t.chars().collect();
    
    if t_vector.len() != s_vector.len() {
        return false;
    }

    s_vector.sort_unstable();
    t_vector.sort_unstable();

        for (index, character) in t_vector.into_iter().enumerate() {
            if character != s_vector[index] {
                return false;
            }
        }
        true
    }
