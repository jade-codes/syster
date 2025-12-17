//! # Generic Operation Framework
//!
//! Provides a middleware-like pattern for operations that produce results and events.
//! This eliminates the need to manually wire up event emission in every component.
//!
//! ## Design
//!
//! ```text
//! ┌─────────────┐
//! │  Operation  │ ──► Returns OperationResult<T, E, Ev>
//! └─────────────┘
//!       │
//!       ▼
//! ┌─────────────┐
//! │  Executor   │ ──► Executes operation
//! └─────────────┘      Publishes event to bus if successful
//!       │              Returns result
//!       ▼
//! ┌─────────────┐
//! │   Result    │
//! └─────────────┘
//! ```
//!
//! ## Benefits
//!
//! - **Separation of concerns**: Business logic doesn't handle event emission
//! - **Consistency**: All operations follow the same pattern
//! - **Testability**: Operations can be tested without event machinery
//! - **Flexibility**: Easy to add logging, metrics, etc. in the middleware layer
//!
//! ## Usage Example
//!
//! ```rust
//! # use syster::core::operation::{OperationResult, EventBus};
//! # use syster::core::events::Event;
//!
//! #[derive(Clone)]
//! enum MyEvent {
//!     ItemAdded { id: usize },
//! }
//!
//! impl Event for MyEvent {}
//!
//! struct MyComponent {
//!     items: Vec<String>,
//! }
//!
//! impl MyComponent {
//!     fn add_item(&mut self, item: String) -> OperationResult<(), String, MyEvent> {
//!         let id = self.items.len();
//!         self.items.push(item);
//!         OperationResult::success((), Some(MyEvent::ItemAdded { id }))
//!     }
//! }
//!
//! struct MyBus;
//! impl EventBus<MyEvent> for MyBus {
//!     fn publish(&mut self, _event: &MyEvent) {}
//! }
//!
//! // Execute operation and handle event
//! let mut component = MyComponent { items: vec![] };
//! let mut event_bus = MyBus;
//! let result = component.add_item("test".to_string());
//!
//! match result.result {
//!     Ok(_) => {
//!         if let Some(event) = result.event {
//!             // Publish to event bus
//!             event_bus.publish(&event);
//!         }
//!     }
//!     Err(_e) => { /* handle error */ }
//! }
//! ```

use super::events::Event;

/// Result of an operation containing the value and an optional event to emit
///
/// This is the return type for all operations that want to participate in
/// automatic event emission.
///
/// # Type Parameters
///
/// - `T`: The success value type
/// - `E`: The error type
/// - `Ev`: The event type (must implement `Event`)
pub struct OperationResult<T, E, Ev: Event> {
    /// The result of the operation (Ok or Err)
    pub result: Result<T, E>,
    /// Optional event to emit if the operation succeeded
    pub event: Option<Ev>,
}

impl<T, E, Ev: Event> OperationResult<T, E, Ev> {
    /// Creates a successful operation result with an optional event
    pub fn success(value: T, event: Option<Ev>) -> Self {
        Self {
            result: Ok(value),
            event,
        }
    }

    /// Creates a failed operation result (no event will be emitted)
    pub fn failure(error: E) -> Self {
        Self {
            result: Err(error),
            event: None,
        }
    }

    /// Converts this operation result into a plain Result, discarding the event
    ///
    /// # Errors
    ///
    /// Returns the error if the operation failed.
    pub fn into_result(self) -> Result<T, E> {
        self.result
    }

    /// Publishes the event if the result is Ok and an event is present
    ///
    /// # Errors
    ///
    /// Returns the error if the operation failed.
    pub fn publish<B: EventBus<Ev>>(self, bus: &mut B) -> Result<T, E> {
        if self.result.is_ok()
            && let Some(event) = self.event
        {
            bus.publish(&event);
        }
        self.result
    }
}

/// Event bus trait for publishing events
///
/// Components can implement this to provide custom event publishing logic.
pub trait EventBus<Ev: Event> {
    /// Publishes an event to all subscribers
    fn publish(&mut self, event: &Ev);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        ItemAdded { id: usize },
    }

    impl Event for TestEvent {}

    struct TestEventBus {
        log: Vec<TestEvent>,
    }

    impl EventBus<TestEvent> for TestEventBus {
        fn publish(&mut self, event: &TestEvent) {
            self.log.push(event.clone());
        }
    }

    #[test]
    fn test_successful_operation_publishes_event() {
        let mut bus = TestEventBus { log: vec![] };

        let result: Result<i32, String> =
            OperationResult::success(42, Some(TestEvent::ItemAdded { id: 0 })).publish(&mut bus);

        assert_eq!(result, Ok(42));
        assert_eq!(bus.log.len(), 1);
        assert_eq!(bus.log[0], TestEvent::ItemAdded { id: 0 });
    }

    #[test]
    fn test_failed_operation_does_not_publish() {
        let mut bus = TestEventBus { log: vec![] };

        let result: Result<(), String> =
            OperationResult::failure("error".to_string()).publish(&mut bus);

        assert!(result.is_err());
        assert_eq!(bus.log.len(), 0);
    }

    #[test]
    fn test_success_without_event() {
        let mut bus = TestEventBus { log: vec![] };

        let result: Result<(), String> = OperationResult::success((), None).publish(&mut bus);

        assert!(result.is_ok());
        assert_eq!(bus.log.len(), 0);
    }
}
