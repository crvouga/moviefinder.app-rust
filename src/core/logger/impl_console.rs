use super::interface::Logger;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConsoleLogger {
    namespace: Vec<String>,
    namespace_str: String,
}

impl ConsoleLogger {
    pub fn new(namespace: Vec<String>) -> ConsoleLogger {
        let namespace_str = namespace
            .iter()
            .map(|x| format!("[{}]", x))
            .collect::<Vec<String>>()
            .join(" ");

        ConsoleLogger {
            namespace,
            namespace_str,
        }
    }
}

impl Logger for ConsoleLogger {
    fn info(&self, message: fmt::Arguments) {
        println!("[info] {} {}\n", self.namespace_str, message);
    }

    fn debug(&self, message: fmt::Arguments) {
        println!("[debug] {} {}\n", self.namespace_str, message);
    }

    fn error(&self, message: fmt::Arguments) {
        println!("[error] {} {}\n", self.namespace_str, message);
    }

    fn child(&self, name: &str) -> Arc<dyn Logger> {
        let mut namespace_new = self.namespace.clone();
        namespace_new.push(name.to_string());
        Arc::new(ConsoleLogger::new(namespace_new))
    }
}
