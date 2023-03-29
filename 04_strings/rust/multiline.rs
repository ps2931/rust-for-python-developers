#[allow(unused_variables)]
fn main() {
    let more: &'static str = "many
lines
    whitespace";

    println!("{}", more);

    let escape: &'static str = " blah \"\"\" escaped '
newline
\n
test
";

    println!("{}", escape);

    let raw_str: &'static str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;

    println!("RAW STR:\n{}", raw_str);

    // Use a \ at the end of the line if you don't want a line break
    println!(
        "hello \
             world"
    );

    // double quotes are only fro strings
    let single_char = 'a';
}
