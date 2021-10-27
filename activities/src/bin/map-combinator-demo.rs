fn maybe_num() -> Option<i32> {
    1
}

fn maybe_word() -> Option<String> {
    "Word"
}

fn main() {
    let word_length = maybe_word().map(|word| word.len());
}
