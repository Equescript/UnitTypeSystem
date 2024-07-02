// pub struct Gradient

// pub fn maximum

pub trait Function<D, R, T = Self> {
    fn function(&self, dom: D) -> R;
}

