struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        // field init shorthand like ES6
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn build_from_user(username: String, email: String, user: &User) -> User {
    User {
        username,
        email,
        ..*user
    }
}


fn main() {
    // create a new user instance
    let first_user = User {
        username: String::from("foo"),
        email: String::from("foo@bar.it"),
        sign_in_count: 1,
        active: true,
    };

    let second_user = build_user(String::from("Naccio"), String::from("n@n.it"));
    let third_user = build_from_user(String::from("Cerza"), String::from("c@c.it"), &second_user);

    println!("First user is {}", first_user.username);
    println!("Second user is {}", second_user.username);
    println!("Third user is {}", third_user.username);
}
