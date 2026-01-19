use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use std::sync::Arc;

pub static MONGO_RUNTIME: Lazy<Arc<Runtime>> = Lazy::new(|| {
    Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cpus())
            .thread_name("gmsv-mongo-worker")
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    )
});

fn num_cpus() -> usize {
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    cpus.clamp(4, 16)
}

pub fn block_on<F>(future: F) -> F::Output
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    MONGO_RUNTIME.block_on(future)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let result = block_on(async { 42 });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_num_cpus() {
        let cpus = num_cpus();
        assert!(cpus >= 4 && cpus <= 16);
    }
}
