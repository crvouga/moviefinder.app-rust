use tokio::task::{self, JoinHandle};

pub async fn join_all<T>(futures: Vec<T>) -> Vec<T::Output>
where
    T: std::future::Future + Send + 'static,
    T::Output: Send + 'static,
{
    let tasks: Vec<JoinHandle<T::Output>> = futures.into_iter().map(task::spawn).collect();

    let mut results = Vec::with_capacity(tasks.len());
    for task in tasks {
        results.push(task.await.ok());
    }

    results.into_iter().flatten().collect()
}
