extern crate concurrent_queue;

use std::env;
use std::sync::mpsc::{channel, Sender};
use std::thread::{ spawn, JoinHandle };
use std::sync::Arc;

use concurrent_queue::lcrq::LCRQ;

const BATCH_SIZE : u64 = 10_000_000;

fn run_channels() {
    fn start_producer(tx: Sender<u64>, start: u64, end: u64) -> JoinHandle<()> {
        spawn(move || {
            for i in start..end {
                tx.send(i);
            }
            println!("Producer {}-{} done", start, end);
        })
    }

    let (tx, rx) = channel();

    let producer_1 = start_producer(tx.clone(), BATCH_SIZE*1, BATCH_SIZE*2);
    let producer_2 = start_producer(tx.clone(), BATCH_SIZE*2, BATCH_SIZE*3);

    let consumer = spawn(move || {
        for _ in 0..(BATCH_SIZE*2) {
            let number = rx.recv().unwrap();
            assert!(number >= BATCH_SIZE);
            assert!(number <  BATCH_SIZE*3);
        }
        println!("Consumer done");
    });

    assert!(producer_1.join().is_ok());
    assert!(producer_2.join().is_ok());
    assert!(consumer.join().is_ok());
}

fn run_queue() {
    fn start_producer(queue: Arc<LCRQ>, start: u64, end: u64) -> JoinHandle<()> {
        spawn(move || {
            for i in start..end {
                queue.enqueue(i);
            }
            println!("Producer {}-{} done", start, end);
        })
    }

    let lcrq = Arc::new(LCRQ::new());

    let producer_1 = start_producer(lcrq.clone(), BATCH_SIZE*1, BATCH_SIZE*2);
    let producer_2 = start_producer(lcrq.clone(), BATCH_SIZE*2, BATCH_SIZE*3);

    let cons_lcrq = lcrq.clone();
    let consumer = spawn(move || {
        for _ in 0..(BATCH_SIZE*2) {
            loop {
                match cons_lcrq.dequeue() {
                    Some(number) => {
                        assert!(number >= BATCH_SIZE);
                        assert!(number <  BATCH_SIZE*3);
                        break;
                    },
                    None => { /* spin */ },
                }
            }
        }
        println!("Consumer done");
    });

    assert!(producer_1.join().is_ok());
    assert!(producer_2.join().is_ok());
    assert!(consumer.join().is_ok());
}

fn print_usage() {
    let app_name = env::args().nth(0).unwrap_or(String::from("speed_test"));
    println!("Usage: {} channel   to test with sync::mpsc::channel, and", app_name);
    println!("       {} queue     to test with the concurrent channel", app_name);
}

fn main() {
    match env::args().nth(1) {
        None => {
            print_usage();
        },
        Some(command) => {
            match command.as_ref() {
                "channel" => run_channels(),
                "queue"   => run_queue(),
                _         => print_usage()
            }
        }
    }
}

