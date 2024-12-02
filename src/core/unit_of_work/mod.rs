use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

struct UnitOfWorkInternal {
    rollback_functions: Vec<
        Box<
            dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send>> + Send,
        >,
    >,
    started: bool,
    committed: bool,
}

impl UnitOfWorkInternal {
    pub fn new() -> UnitOfWorkInternal {
        Self {
            rollback_functions: Vec::new(),
            started: false,
            committed: false,
        }
    }

    pub async fn start(&mut self) -> Result<(), std::io::Error> {
        if self.started {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Transaction already started",
            ));
        }
        self.started = true;
        Ok(())
    }

    pub async fn commit(&mut self) -> Result<(), std::io::Error> {
        if !self.started {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Transaction not started",
            ));
        }
        if self.committed {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Transaction already committed",
            ));
        }
        self.rollback_functions.clear(); // Clear rollback functions on commit
        self.committed = true;
        Ok(())
    }

    pub async fn rollback(&mut self) -> Result<(), std::io::Error> {
        if !self.started {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Transaction not started",
            ));
        }
        if self.committed {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Transaction already committed",
            ));
        }
        while let Some(rollback_single) = self.rollback_functions.pop() {
            rollback_single().await?;
        }
        Ok(())
    }

    pub fn register_rollback<F, Fut>(&mut self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), std::io::Error>> + Send + 'static,
    {
        self.rollback_functions.push(Self::box_future(f));
    }

    fn box_future<F, Fut>(
        f: F,
    ) -> Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send>> + Send>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), std::io::Error>> + Send + 'static,
    {
        Box::new(move || Box::pin(f()))
    }
}

pub struct UnitOfWork(Arc<Mutex<UnitOfWorkInternal>>);

impl UnitOfWork {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(UnitOfWorkInternal::new())))
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        self.0.lock().await.start().await
    }

    pub async fn commit(&self) -> Result<(), std::io::Error> {
        self.0.lock().await.commit().await
    }

    pub async fn rollback(&self) -> Result<(), std::io::Error> {
        self.0.lock().await.rollback().await
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    pub async fn register_rollback<F, Fut>(&self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), std::io::Error>> + Send + 'static,
    {
        self.0.lock().await.register_rollback(f);
    }

    pub async fn transact<'a, F>(
        transaction: impl FnOnce(UnitOfWork) -> F + 'a,
    ) -> Result<(), std::io::Error>
    where
        F: Future<Output = Result<(), std::io::Error>> + Send + 'a,
    {
        let uow = UnitOfWork::new();
        uow.start().await?;

        let result = transaction(uow.clone()).await;

        if let Err(_) = result {
            uow.rollback().await?;
            result
        } else {
            uow.commit().await?;
            result
        }
    }
}
