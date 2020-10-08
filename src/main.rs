trait State {
    type Trigger;

    fn next(self, t: Self::Trigger) -> Self;
}

#[derive(Debug, Clone, Copy)]
enum Switch {
    Off,
    HalfOn,
    On,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    FlickSwitch,
    TurnSwitch,
}

impl State for Switch {
    type Trigger = Action;
    fn next(self, t: Self::Trigger) -> Self {
        match self {
            Switch::Off => match t {
                Action::FlickSwitch => Switch::On,
                Action::TurnSwitch => Switch::HalfOn,
            },
            Switch::HalfOn => Switch::On,
            Switch::On => Switch::Off,
        }
    }
}

fn main() {
    let s = Switch::Off;

    println!("{:?}", s);
    println!("{:?}", s.next(Action::TurnSwitch));
    println!("{:?}", s.next(Action::FlickSwitch));
    println!("{:?}", s.next(Action::TurnSwitch).next(Action::TurnSwitch));
}
