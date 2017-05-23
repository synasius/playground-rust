fn main() {
    let s1 = String::from("Hello!");
    let mut s1 = take_ownership(s1);

    change(&mut s1);

    let l = calculate_length(&s1);

    println!("The len of '{}' is: {}", s1, l);
    println!("The first word is {}", first_word(&s1));
}

fn take_ownership(value: String) -> String {
    println!("{}", value);
    value
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// string slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
