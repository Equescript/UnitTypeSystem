use super::property::Singleton;

// static mut UPDATE_TIMER: Option<UpdateTimer> = None;

#[derive(Clone, Copy)]
enum State<T> {
    Updated(T),
    Outdated,
}

#[derive(Clone, Copy)]
pub struct DependentData<T> {
    data: T,
    // update_time: UpdateTime,
    // dependencies: Vec<>,
}

impl<T> DependentData<T> {
    pub fn data(&self) -> &T {
        &self.data
    }
    pub fn update(&mut self, data: T) {
        self.data = data;
    }
}

impl<T> std::ops::Deref for DependentData<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/* impl<T> std::ops::DerefMut for DependentData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
} */

/* #[derive(Clone, Copy)]
pub struct UpdateTime {
    time: usize,
}

#[derive(Clone, Copy)]
pub struct UpdateTimer {
    time: usize,
}

impl UpdateTimer {
    fn time(&self) -> UpdateTime {
        UpdateTime { time: self.time }
    }
    fn update(&mut self) {
        self.time += 1;
    }
}

impl Singleton for UpdateTimer {
    fn instance() -> &'static Self {
        unsafe { match &UPDATE_TIMER {
            Some(t) => t,
            None => panic!(),
        } }
    }
    fn instance_mut() -> &'static mut Self {
        unsafe { match &mut UPDATE_TIMER {
            Some(t) => t,
            None => panic!(),
        } }
    }
} */