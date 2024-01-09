# Gamo

Create a Range like struct of user define types for easy for loop. *Gamo* means range in Esperanto.

# What is it?

Currently Rust does not have a stable API to create `Range<T>` of user defined type `T`. This crate provides a `Range` like struct `Gamo` that can be easily used with for-loops.

# Usage

```toml
[dependencies]
gamo = "0.2.0"
```

The type `T` used in `Gamo<T>` must implement `IntoNext` trait.

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
