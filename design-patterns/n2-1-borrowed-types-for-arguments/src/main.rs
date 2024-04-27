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

#[allow(dead_code)]
fn main0() {
    let ferris: String = "Ferris".to_string();
    let curious: String = "Curious".to_string();

    println!("{}: {}", ferris, three_vowels(&ferris)); // param is String type
    println!("{}: {}", curious, three_vowels(&curious)); // param is String type

    // This works fine, but the following two lines would fail
    // if func param type was String (rather than str):
    println!("Ferris: {}", three_vowels("Ferris")); // param is str type
    println!("Curious: {}", three_vowels("Curious")); // param is str type
}

fn main() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris who was asked to present an _iou_!".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{word} has three consecutive vowels!");
        }
    }
}