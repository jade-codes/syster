//! Generic debouncer for batching requests by key
//!
//! Waits for a quiet period before emitting ready items.

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

use tokio::sync::mpsc;

/// Default debounce delay (ms)
pub const DEFAULT_DELAY_MS: u64 = 150;

/// Spawn a debounce task that batches requests by key
///
/// When a key is received, waits `delay` before emitting it.
/// If the same key arrives again, the timer resets.
pub fn spawn<K, F>(delay: Duration, mut rx: mpsc::UnboundedReceiver<K>, mut emit: F)
where
    K: Eq + Hash + Clone + Send + 'static,
    F: FnMut(K) -> bool + Send + 'static,
{
    tokio::spawn(async move {
        let mut pending: HashMap<K, tokio::time::Instant> = HashMap::new();

        loop {
            if pending.is_empty() {
                match rx.recv().await {
                    Some(key) => {
                        pending.insert(key, tokio::time::Instant::now());
                    }
                    None => break,
                }
            } else {
                let oldest = pending.values().min().cloned().unwrap();
                let time_until_ready = delay.saturating_sub(oldest.elapsed());

                tokio::select! {
                    biased;

                    _ = tokio::time::sleep(time_until_ready) => {
                        let now = tokio::time::Instant::now();
                        let ready: Vec<K> = pending
                            .iter()
                            .filter(|(_, time)| now.duration_since(**time) >= delay)
                            .map(|(key, _)| key.clone())
                            .collect();

                        for key in ready {
                            pending.remove(&key);
                            if !emit(key) {
                                return;
                            }
                        }
                    }

                    maybe_key = rx.recv() => {
                        match maybe_key {
                            Some(key) => {
                                pending.insert(key, tokio::time::Instant::now());
                            }
                            None => break,
                        }
                    }
                }
            }
        }
    });
}
