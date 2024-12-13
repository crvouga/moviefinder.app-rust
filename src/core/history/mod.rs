use serde::{Deserialize, Serialize};

/// `History` is a data structure that allows you to keep track of the state of
/// an object over time. It is useful for implementing undo/redo functionality.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History<T> {
    past: Vec<T>,
    present: T,
    future: Vec<T>,
}

impl<T> History<T>
where
    T: Clone,
{
    /// Creates a new history with the given present state.
    pub fn new(present: T) -> Self {
        Self {
            past: Vec::new(),
            present,
            future: Vec::new(),
        }
    }

    /// Clears the history.
    pub fn clear(&mut self, present: T) {
        self.past.clear();
        self.present = present;
        self.future.clear();
    }

    /// Pushes a new state onto the history.
    pub fn push(&mut self, new_present: T) {
        self.past.push(self.present.clone());
        self.present = new_present;
        self.future.clear();
    }

    /// Changes the current state of the history to the previous state.
    pub fn undo(&mut self) -> bool {
        if let Some(last) = self.past.pop() {
            self.future.insert(0, self.present.clone());
            self.present = last;
            true
        } else {
            false // Cannot undo
        }
    }

    /// Changes the current state of the history to the next state.
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.future.get(0) {
            self.past.push(self.present.clone());
            self.present = next.clone();
            self.future.remove(0);
            true
        } else {
            false // Cannot redo
        }
    }

    /// Returns the current state of the history.
    pub fn present(&self) -> &T {
        &self.present
    }

    /// Returns true if the history can be redone.
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Returns true if the history can be undone.
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history() {
        let mut history = History::new(0);

        history.push(1);
        history.push(2);
        assert_eq!(history.present(), &2);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.undo();
        assert_eq!(history.present(), &1);
        assert!(history.can_undo());
        assert!(history.can_redo());

        history.undo();
        assert_eq!(history.present(), &0);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        history.redo();
        assert_eq!(history.present(), &1);
        assert!(history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_undo_multiple_times() {
        let mut history = History::new(0);

        // Push multiple states
        history.push(1);
        history.push(2);
        history.push(3);

        // Ensure present is the latest state
        assert_eq!(history.present(), &3);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo once
        assert!(history.undo());
        assert_eq!(history.present(), &2);
        assert!(history.can_undo());
        assert!(history.can_redo());

        // Undo twice
        assert!(history.undo());
        assert_eq!(history.present(), &1);
        assert!(history.can_undo());
        assert!(history.can_redo());

        // Undo thrice
        assert!(history.undo());
        assert_eq!(history.present(), &0);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Further undo should return false
        assert!(!history.undo());
        assert_eq!(history.present(), &0);
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }
}
