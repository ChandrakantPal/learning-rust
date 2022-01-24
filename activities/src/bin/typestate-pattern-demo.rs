struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}
