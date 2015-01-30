pub use self::future::Future;

mod future;

pub fn run<F, R>(f: F) -> Future<R> where F: FnOnce()->R, F: Send, R: Send {
    Future::from_fn(f)
}