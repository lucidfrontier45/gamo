pub trait TryToNext: Sized {
    fn try_to_next(&self) -> Option<Self>;
}

pub struct Gamo<T> {
    current: Option<T>,
    end: T,
    inclusive: bool,
}

impl<T: TryToNext> Gamo<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            current: Some(start),
            end,
            inclusive: false,
        }
    }
    pub fn new_inclusive(start: T, end: T) -> Self {
        Self {
            current: Some(start),
            end,
            inclusive: true,
        }
    }
}

impl<T> Iterator for Gamo<T>
where
    T: TryToNext + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(v) = self.current.take() else {
            return None;
        };

        if v < self.end || (self.inclusive && v == self.end) {
            self.current = v.try_to_next();
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Gamo, TryToNext};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TimeSlot(usize);

    impl TryToNext for TimeSlot {
        fn try_to_next(&self) -> Option<Self> {
            Some(Self(self.0 + 1))
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

    #[test]
    fn test_gamo_inclusive() {
        let mut r = Gamo::new_inclusive(TimeSlot(0), TimeSlot(5));
        assert_eq!(r.next(), Some(TimeSlot(0)));
        assert_eq!(r.next(), Some(TimeSlot(1)));
        assert_eq!(r.next(), Some(TimeSlot(2)));
        assert_eq!(r.next(), Some(TimeSlot(3)));
        assert_eq!(r.next(), Some(TimeSlot(4)));
        assert_eq!(r.next(), Some(TimeSlot(5)));
        assert_eq!(r.next(), None);
    }
}
