pub trait Logger {
    fn info(&self, message: &str) -> ();
    fn debug(&self, message: &str) -> ();
    fn error(&self, message: &str) -> ();
}
