struct Test {
    score: i32,
}
fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 80 },
        Test { score: 70 },
        Test { score: 60 },
    ];

    for test in my_scores {
        println!("score: {:?}", test.score)
    }
}
