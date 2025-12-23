//! # Generic Event System
//!
//! Provides a reusable publish-subscribe event system for the entire codebase.
//!
//! ## Design
//!
//! - **Event**: Any type implementing the `Event` trait (typically enums)
//! - **EventEmitter<T, Context>**: Manages listeners and emits events of type T
//! - **EventListener<T, Context>**: Callback function that responds to events
//!
//! ## Usage Example
//!
//! ```rust
//! use syster::core::events::{Event, EventEmitter};
//!
//! #[derive(Debug, Clone, PartialEq)]
//! enum MyEvent {
//!     ItemAdded { id: u32 },
//!     ItemRemoved { id: u32 },
//! }
//!
//! impl Event for MyEvent {}
//!
//! struct MyComponent {
//!     items: Vec<u32>,
//!     events: EventEmitter<MyEvent, MyComponent>,
//! }
//!
//! impl MyComponent {
//!     fn new() -> Self {
//!         Self {
//!             items: Vec::new(),
//!             events: EventEmitter::new(),
//!         }
//!     }
//!
//!     fn subscribe<F>(&mut self, listener: F)
//!     where
//!         F: Fn(&MyEvent, &mut Self) + Send + Sync + 'static,
//!     {
//!         self.events.subscribe(listener);
//!     }
//!
//!     fn add_item(&mut self, id: u32) {
//!         self.items.push(id);
//!         let events = std::mem::take(&mut self.events);
//!         self.events = events.emit(MyEvent::ItemAdded { id }, self);
//!     }
//! }
//! ```

use std::sync::Arc;

/// Marker trait for event types
///
/// Events should typically be enums representing different state changes.
/// They must be cloneable so they can be passed to multiple listeners.
pub trait Event: Clone {}

/// Type alias for event listener callbacks
///
/// Listeners receive a reference to the event and mutable access to the emitter's context.
/// This allows listeners to trigger state changes in response to events.
pub type EventListener<T, Context> = Arc<dyn Fn(&T, &mut Context) + Send + Sync>;

/// Manages event listeners and emission for a specific event type
///
/// # Type Parameters
///
/// - `T`: The event type (must implement `Event`)
/// - `Context`: The context type passed to listeners (typically the owning struct)
///
/// # Thread Safety
///
/// EventEmitter is not thread-safe by default. If you need concurrent access,
/// wrap it in a `Mutex` or `RwLock`.
pub struct EventEmitter<T: Event, Context> {
    listeners: Vec<EventListener<T, Context>>,
}

impl<T: Event, Context> std::fmt::Debug for EventEmitter<T, Context> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventEmitter")
            .field("listener_count", &self.listeners.len())
            .finish()
    }
}

impl<T: Event, Context> EventEmitter<T, Context> {
    /// Creates a new event emitter with no listeners
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    /// Subscribes a new listener to this event emitter
    ///
    /// The listener will be called whenever `emit()` is invoked.
    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(&T, &mut Context) + Send + Sync + 'static,
    {
        self.listeners.push(Arc::new(listener));
    }

    /// Emits an event to all registered listeners
    ///
    /// Listeners are called in the order they were registered.
    /// Each listener receives a reference to the event and mutable access to the context.
    ///
    /// Note: This consumes the event emitter temporarily to avoid borrow checker issues.
    /// The emitter must be taken out, used to emit, then put back.
    pub fn emit(self, event: T, context: &mut Context) -> Self {
        // Clone listeners to avoid borrow issues
        let listeners: Vec<_> = self.listeners.iter().cloned().collect();

        for listener in listeners {
            listener(&event, context);
        }

        self
    }

    /// Returns the number of registered listeners
    pub fn listener_count(&self) -> usize {
        self.listeners.len()
    }

    /// Clears all registered listeners
    pub fn clear_listeners(&mut self) {
        self.listeners.clear();
    }
}

impl<T: Event, Context> Default for EventEmitter<T, Context> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        ValueChanged { old: i32, new: i32 },
        Reset,
    }

    impl Event for TestEvent {}

    struct TestContext {
        value: i32,
        event_log: Vec<String>,
        emitter: EventEmitter<TestEvent, TestContext>,
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                value: 0,
                event_log: Vec::new(),
                emitter: EventEmitter::new(),
            }
        }

        fn set_value(&mut self, new_value: i32) {
            let old = self.value;
            self.value = new_value;
            let emitter = std::mem::take(&mut self.emitter);
            self.emitter = emitter.emit(
                TestEvent::ValueChanged {
                    old,
                    new: new_value,
                },
                self,
            );
        }

        fn reset(&mut self) {
            self.value = 0;
            let emitter = std::mem::take(&mut self.emitter);
            self.emitter = emitter.emit(TestEvent::Reset, self);
        }
    }

    #[test]
    fn test_event_emitter_creation() {
        let emitter: EventEmitter<TestEvent, TestContext> = EventEmitter::new();
        assert_eq!(emitter.listener_count(), 0);
    }

    #[test]
    fn test_subscribe_and_emit() {
        let mut context = TestContext::new();

        context.emitter.subscribe(|event, ctx| {
            ctx.event_log.push(format!("Event received: {event:?}"));
        });

        assert_eq!(context.emitter.listener_count(), 1);

        context.set_value(42);

        assert_eq!(context.event_log.len(), 1);
        assert!(context.event_log[0].contains("ValueChanged"));
    }

    #[test]
    fn test_multiple_listeners() {
        let mut context = TestContext::new();

        context.emitter.subscribe(|_event, ctx| {
            ctx.event_log.push("Listener 1".to_string());
        });

        context.emitter.subscribe(|_event, ctx| {
            ctx.event_log.push("Listener 2".to_string());
        });

        assert_eq!(context.emitter.listener_count(), 2);

        context.set_value(100);

        assert_eq!(context.event_log.len(), 2);
        assert_eq!(context.event_log[0], "Listener 1");
        assert_eq!(context.event_log[1], "Listener 2");
    }

    #[test]
    fn test_clear_listeners() {
        let mut context = TestContext::new();

        context.emitter.subscribe(|_event, ctx| {
            ctx.event_log.push("Should not be called".to_string());
        });

        context.emitter.clear_listeners();
        assert_eq!(context.emitter.listener_count(), 0);

        context.set_value(50);
        assert_eq!(context.event_log.len(), 0);
    }

    #[test]
    fn test_different_event_types() {
        let mut context = TestContext::new();

        context.emitter.subscribe(|event, ctx| match event {
            TestEvent::ValueChanged { old, new } => {
                ctx.event_log.push(format!("Changed from {old} to {new}"));
            }
            TestEvent::Reset => {
                ctx.event_log.push("Reset".to_string());
            }
        });

        context.set_value(10);
        context.set_value(20);
        context.reset();

        assert_eq!(context.event_log.len(), 3);
        assert_eq!(context.event_log[0], "Changed from 0 to 10");
        assert_eq!(context.event_log[1], "Changed from 10 to 20");
        assert_eq!(context.event_log[2], "Reset");
    }
}
