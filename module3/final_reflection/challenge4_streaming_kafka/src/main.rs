use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    id: u64,
    timestamp: u64,
    data: String,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
struct ProcessingStats {
    total_processed: usize,
    total_filtered: usize,
    total_aggregated: i32,
}

#[tokio::main]
async fn main() {
    println!("=== Streaming Data Processing with Parallel Workers ===\n");

    // Shared statistics
    let total_processed = Arc::new(AtomicUsize::new(0));
    let total_filtered = Arc::new(AtomicUsize::new(0));
    let total_sum = Arc::new(AtomicUsize::new(0));

    // Create message channel
    let (tx, rx) = mpsc::channel::<Message>(100);

    // Spawn producer task
    let producer_handle = tokio::spawn(async move {
        produce_messages(tx).await;
    });

    // Spawn multiple worker tasks
    let total_processed_clone = Arc::clone(&total_processed);
    let total_filtered_clone = Arc::clone(&total_filtered);
    let total_sum_clone = Arc::clone(&total_sum);

    let consumer_handle = tokio::spawn(async move {
        consume_messages(
            rx,
            total_processed_clone,
            total_filtered_clone,
            total_sum_clone,
        )
        .await;
    });

    // Wait for completion
    let _ = producer_handle.await;
    let _ = consumer_handle.await;

    // Print final statistics
    println!("\n=== Final Statistics ===");
    println!("Total messages processed: {}", total_processed.load(Ordering::SeqCst));
    println!("Messages filtered (value > 50): {}", total_filtered.load(Ordering::SeqCst));
    println!("Sum of aggregated values: {}", total_sum.load(Ordering::SeqCst));
}

async fn produce_messages(tx: mpsc::Sender<Message>) {
    println!("Producer: Starting to generate messages...\n");

    for i in 0..20 {
        let message = Message {
            id: i as u64,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: format!("Event_{}", i),
            value: (i * 7 + 13) % 100, // Pseudo-random value
        };

        println!("Producer: Sending message {}: {:?}", i, message);

        if tx.send(message).await.is_err() {
            eprintln!("Producer: Channel closed, stopping production");
            break;
        }

        // Simulate delay in message generation
        sleep(Duration::from_millis(100)).await;
    }

    println!("\nProducer: Finished sending all messages");
}

async fn consume_messages(
    mut rx: mpsc::Receiver<Message>,
    total_processed: Arc<AtomicUsize>,
    total_filtered: Arc<AtomicUsize>,
    total_sum: Arc<AtomicUsize>,
) {
    println!("Consumer: Starting message consumption with parallel workers\n");

    // Spawn worker tasks that process messages
    let num_workers = 3;
    let mut worker_handles = vec![];

    for worker_id in 0..num_workers {
        let total_processed_clone = Arc::clone(&total_processed);
        let total_filtered_clone = Arc::clone(&total_filtered);
        let total_sum_clone = Arc::clone(&total_sum);

        let handle = tokio::spawn(async move {
            process_worker(
                worker_id,
                total_processed_clone,
                total_filtered_clone,
                total_sum_clone,
            )
            .await;
        });

        worker_handles.push(handle);
    }

    // Distribute messages to workers using round-robin
    let mut worker_idx = 0;
    let mut worker_senders = vec![];

    for worker_id in 0..num_workers {
        let (tx, rx) = mpsc::channel::<Message>(50);
        worker_senders.push(tx);

        let total_processed_clone = Arc::clone(&total_processed);
        let total_filtered_clone = Arc::clone(&total_filtered);
        let total_sum_clone = Arc::clone(&total_sum);

        tokio::spawn(async move {
            let mut rx = rx;
            while let Some(msg) = rx.recv().await {
                process_message(
                    worker_id,
                    &msg,
                    &total_processed_clone,
                    &total_filtered_clone,
                    &total_sum_clone,
                )
                .await;
            }
        });
    }

    // Distribute messages to workers
    while let Some(message) = rx.recv().await {
        let sender = &worker_senders[worker_idx];
        if sender.send(message).await.is_err() {
            eprintln!("Consumer: Worker channel closed");
            break;
        }
        worker_idx = (worker_idx + 1) % num_workers;
    }

    println!("\nConsumer: All messages processed");
}

async fn process_worker(
    worker_id: usize,
    total_processed: Arc<AtomicUsize>,
    total_filtered: Arc<AtomicUsize>,
    total_sum: Arc<AtomicUsize>,
) {
    println!("Worker {}: Ready", worker_id);
    // Worker loop handled in consume_messages spawn
}

async fn process_message(
    worker_id: usize,
    message: &Message,
    total_processed: &Arc<AtomicUsize>,
    total_filtered: &Arc<AtomicUsize>,
    total_sum: &Arc<AtomicUsize>,
) {
    // Simulate processing
    sleep(Duration::from_millis(50)).await;

    total_processed.fetch_add(1, Ordering::SeqCst);

    if message.value > 50 {
        total_filtered.fetch_add(1, Ordering::SeqCst);
        total_sum.fetch_add(message.value as usize, Ordering::SeqCst);
        println!(
            "Worker {}: ✓ FILTERED (value={}) - {}",
            worker_id, message.value, message.data
        );
    } else {
        println!(
            "Worker {}: - SKIPPED (value={}) - {}",
            worker_id, message.value, message.data
        );
    }
}
