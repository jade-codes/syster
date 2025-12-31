//! Tests for background tasks

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use tokio::sync::mpsc;

use crate::server::background_tasks::debounce;

/// Test that debounce waits before emitting
#[tokio::test]
async fn test_debounce_waits_before_emit() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let emitted = Arc::new(AtomicUsize::new(0));
    let emitted_clone = emitted.clone();

    debounce::spawn(Duration::from_millis(50), rx, move |_key| {
        emitted_clone.fetch_add(1, Ordering::SeqCst);
        true
    });

    tx.send("doc1".to_string()).unwrap();

    // Should not have emitted yet
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(
        emitted.load(Ordering::SeqCst),
        0,
        "Should not emit immediately"
    );

    // Wait for debounce
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert_eq!(emitted.load(Ordering::SeqCst), 1, "Should emit after delay");
}

/// Test that rapid changes reset the debounce timer
#[tokio::test]
async fn test_debounce_resets_on_same_key() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let emitted = Arc::new(AtomicUsize::new(0));
    let emitted_clone = emitted.clone();

    debounce::spawn(Duration::from_millis(50), rx, move |_key| {
        emitted_clone.fetch_add(1, Ordering::SeqCst);
        true
    });

    // Send same key multiple times rapidly
    tx.send("doc1".to_string()).unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    tx.send("doc1".to_string()).unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    tx.send("doc1".to_string()).unwrap();

    // Timer keeps resetting, so still 0
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(emitted.load(Ordering::SeqCst), 0, "Timer should reset");

    // Now wait for full debounce period
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert_eq!(
        emitted.load(Ordering::SeqCst),
        1,
        "Should emit once after quiet period"
    );
}

/// Test that different keys are tracked independently
#[tokio::test]
async fn test_debounce_independent_keys() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let emitted = Arc::new(AtomicUsize::new(0));
    let emitted_clone = emitted.clone();

    debounce::spawn(Duration::from_millis(50), rx, move |_key| {
        emitted_clone.fetch_add(1, Ordering::SeqCst);
        true
    });

    // Send two different keys
    tx.send("doc1".to_string()).unwrap();
    tx.send("doc2".to_string()).unwrap();

    // Wait for debounce
    tokio::time::sleep(Duration::from_millis(70)).await;
    assert_eq!(emitted.load(Ordering::SeqCst), 2, "Both keys should emit");
}

/// Test that dropping the sender stops the debouncer
#[tokio::test]
async fn test_debounce_stops_on_channel_close() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let emitted = Arc::new(AtomicUsize::new(0));
    let emitted_clone = emitted.clone();

    debounce::spawn(Duration::from_millis(50), rx, move |_key| {
        emitted_clone.fetch_add(1, Ordering::SeqCst);
        true
    });

    tx.send("doc1".to_string()).unwrap();

    // Drop sender before debounce fires
    drop(tx);

    // The pending item may or may not emit depending on timing
    // but the task should terminate cleanly without panic
    tokio::time::sleep(Duration::from_millis(100)).await;
}

/// Test that emit returning false stops the debouncer
#[tokio::test]
async fn test_debounce_stops_on_emit_false() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let emitted = Arc::new(AtomicUsize::new(0));
    let emitted_clone = emitted.clone();

    debounce::spawn(Duration::from_millis(30), rx, move |_key| {
        emitted_clone.fetch_add(1, Ordering::SeqCst);
        false // Signal to stop
    });

    tx.send("doc1".to_string()).unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // First emit happened
    assert_eq!(emitted.load(Ordering::SeqCst), 1);

    // Send another - may fail because receiver dropped, which is expected
    let result = tx.send("doc2".to_string());
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Either send failed OR it succeeded but wasn't processed
    if result.is_ok() {
        assert_eq!(
            emitted.load(Ordering::SeqCst),
            1,
            "Should not process after stop"
        );
    }
    // If send failed, that's also correct - task stopped and dropped receiver
}
