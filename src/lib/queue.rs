use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Create a WorkQueue of any type that holds all the work to be done
///
/// # Example
///
/// ```ignore
/// let max_work_async: usize = 3;
/// 
/// // Check if there is work to be done
/// while queue_clone.length().unwrap() > 0 {
///     let mut tasks = Vec::new();
///     let mut current_work: usize = 0;
///
///     // Don't overwork your PC. Set a maximum number of tasks async
///     while current_work < max_work_async {
///         if let Some(work) = queue_clone.get_work() {
///             let task = tokio::task::spawn(complex_calculation(work, 1000));
///             tasks.push(task);
///             current_work += 1;
///         } else {
///             break;
///         }
///     }
/// 
///     for task in tasks {
///         let result = task.await.unwrap();
///         tx_clone.send(result).unwrap();
///     }
/// 
///     current_work = 0;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Queue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Add work to the queue
    pub fn add_work(&self, work: T) -> Result<(), ()> {
        let queue = self.queue.lock();

        if let Ok(mut q) = queue {
            q.push_back(work);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Get the first available work
    pub fn get_work(&self) -> Option<T> {
        // Lock the queue to fetch a work to do and prevent other threads from
        // fetching the same work.
        let queue = self.queue.lock();

        if let Ok(mut q) = queue {
            // Remove the first work available
            // Follows the the FIFO layout
            q.pop_front()
        } else {
            None
        }
    }

    /// Count the work left
    pub fn length(&self) -> Option<usize> {
        let queue = self.queue.lock();

        if let Ok(q) = queue {
            Some(q.len())
        } else {
            None
        }
    }
}

