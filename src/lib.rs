pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<T>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
