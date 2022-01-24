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

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

fn main() {
    let employee = Employee::new("Sanjay");
    let onboarded = employee.read_agreement();
}
