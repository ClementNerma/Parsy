pub trait Container<T> {
    fn create() -> Self;
    fn with_capacity(size: usize) -> Self;
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;

    fn add(&mut self, value: T);
    fn append(&mut self, values: &mut Vec<T>);
    fn append_iter<I: IntoIterator<Item = T>>(&mut self, iter: I);

    fn prepend(self, value: T) -> Self;
}

impl<T> Container<T> for Vec<T> {
    fn create() -> Self {
        vec![]
    }

    fn with_capacity(size: usize) -> Self {
        Vec::with_capacity(size)
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    fn add(&mut self, value: T) {
        Vec::push(self, value);
    }

    fn append(&mut self, values: &mut Vec<T>) {
        Vec::append(self, values);
    }

    fn append_iter<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        Vec::extend(self, iter)
    }

    fn prepend(self, value: T) -> Self {
        let mut out = Vec::with_capacity(1 + self.len());
        out.push(value);
        out.extend(self);
        out
    }
}

#[derive(Clone, Copy)]
pub struct NoAllocContainer;

impl<T> Container<T> for NoAllocContainer {
    fn create() -> Self {
        Self
    }

    fn with_capacity(_: usize) -> Self {
        Self
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Trigger the iterator to trigger panics etc.
        iter.into_iter().count();
        Self
    }

    fn add(&mut self, _: T) {}
    fn append(&mut self, _: &mut Vec<T>) {}
    fn append_iter<I: IntoIterator<Item = T>>(&mut self, _: I) {}

    fn prepend(self, _: T) -> Self {
        Self
    }
}
