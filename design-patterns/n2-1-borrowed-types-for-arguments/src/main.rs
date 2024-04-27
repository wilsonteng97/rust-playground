// example where we wish to determine if a word contains three consecutive vowels.
// We don’t need to own the string to determine this, so we will take a reference.


fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for (_i, c) in word.chars().enumerate()
    {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count == 3 {
                    return true;
                }
            }
            _ => {
                vowel_count = 0;
            }
        };
        
    };
    false
}

fn main() {
    let ferris: String = "Ferris".to_string();
    let curious: String = "Curious".to_string();

    println!("{}: {}", ferris, three_vowels(&ferris)); // param is String type
    println!("{}: {}", curious, three_vowels(&curious)); // param is String type

    // This works fine, but the following two lines would fail
    // if func param type was String (rather than str):
    println!("Ferris: {}", three_vowels("Ferris")); // param is str type
    println!("Curious: {}", three_vowels("Curious")); // param is str type
}