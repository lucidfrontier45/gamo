use gamo::{Gamo, IntoNext};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TimeSlot(usize);

impl IntoNext for TimeSlot {
    fn into_next(self) -> Self {
        Self(self.0 + 1)
    }
}

fn main() {
    for t in Gamo::new(TimeSlot(0), TimeSlot(5)) {
        println!("{:?}", t);
    }
}
