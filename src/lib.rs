pub trait IntoNext {
    fn into_next(self) -> Self;
}

pub struct Gamo<T> {
    current: T,
    end: T,
}

impl<T> Gamo<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl<T> Iterator for Gamo<T>
where
    T: IntoNext + PartialOrd + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let current = self.current.clone();
            self.current = current.clone().into_next();
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Gamo, IntoNext};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TimeSlot(usize);

    impl IntoNext for TimeSlot {
        fn into_next(self) -> Self {
            Self(self.0 + 1)
        }
    }

    #[test]
    fn test_gamo() {
        let mut r = Gamo::new(TimeSlot(0), TimeSlot(5));
        assert_eq!(r.next(), Some(TimeSlot(0)));
        assert_eq!(r.next(), Some(TimeSlot(1)));
        assert_eq!(r.next(), Some(TimeSlot(2)));
        assert_eq!(r.next(), Some(TimeSlot(3)));
        assert_eq!(r.next(), Some(TimeSlot(4)));
        assert_eq!(r.next(), None);
    }
}
