use gamo::{Gamo, TryToNext};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TimeSlot(usize);

impl TryToNext for TimeSlot {
    fn try_to_next(&self) -> Option<Self> {
        Some(Self(self.0 + 1))
    }
}

fn main() {
    for t in Gamo::new(TimeSlot(0), TimeSlot(5)) {
        println!("{:?}", t);
    }
}
