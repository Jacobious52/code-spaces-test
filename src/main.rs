trait State {
    fn next(self) -> Self;
}

#[derive(Debug, Clone, Copy)]
enum Switch {
    Off,
    On
}

impl State for Switch {
    fn next(self) -> Self {
        match self {
            Switch::Off => Switch::On,
            Switch::On => Switch::Off,
        }
    }
}

fn main() {
    let s = Switch::Off;

    println!("{:?}", s);
    println!("{:?}", s.next());
    println!("{:?}", s.next().next());
    println!("{:?}", s.next().next().next());
}
