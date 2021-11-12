fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user={:?}}", user),
        None => println!("no user"),
    }

    if let Some(user) = maybe_user {
        println!("user={:?}}", user);
    } else {
        println!("no user");
    }
}
