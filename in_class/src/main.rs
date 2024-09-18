// fn concat_strings(s1: &String, s2: &String) -> String {
//     let mut new_word = (*s1).clone();
//     new_word.push_str(string: s2);
//     new_word;

//     // Your code here
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//     let result = concat_strings(&s1, &s2);
//     println!("{}", result); // Should print: "Hello, World!"
// }

fn clone_and_modify(s: &String) -> String {
    // Your code here
    s.clone();
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
