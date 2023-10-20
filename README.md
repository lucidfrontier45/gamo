# Gamo

Create Range like struct of user define types for easy for loop. *Gamo* means range in Esperanto.

# What is it?

Currently in Rust there is not stable API to create `Range<T>` for user defined type `T`. This crates provide a `Range` like struct `Gamo` for easily use with for loops.

# Usage

```toml
[dependencies]
gamo = "0.1.0"
```

The type `T` used in `Gamo<T>` must implements `IntoNext` trait.

# Example

```rust
use gamo::{IntoNext, Gamo};

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
```
