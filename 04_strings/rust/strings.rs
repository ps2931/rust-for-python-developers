fn main() {
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
    println!("Bob says again: {}", bob);

    let pangram: &'static str = "the quick brown fox jumps over the lazy";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    // a slice of chars
    let chars_to_trim: &[char] = &[' ', ','];
    // remove chars_to_trim chars from string if present as prefix or suffix
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used    characters: {}", string);
    println!("Trimmed characters: {}", trimmed_str);

    // few more examples of trim_matches
    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar"); // Pass
    assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar"); // Pass

    let x: &[_] = &['1', '2'];
    assert_eq!("12foo1bar12".trim_matches(x), "foo1bar"); // Pass
}
