extern crate concurrent_queue;

use std::env;
use std::sync::mpsc::{channel, Sender};
use std::thread::{ spawn, JoinHandle };
use std::sync::Arc;

use concurrent_queue::lcrq::LCRQ;

const BATCH_SIZE : u64 = 10_000_000;

fn run_channels(num_producers: u64) {
    fn start_producer(tx: Sender<u64>, start: u64, end: u64) -> JoinHandle<()> {
        spawn(move || {
            for i in start..end {
                tx.send(i).unwrap();
            }
            println!("Producer {}-{} done", start, end);
        })
    }

    let (tx, rx) = channel();

    let producers = (0..num_producers)
        .map(|i| start_producer(tx.clone(), BATCH_SIZE*(i+1), BATCH_SIZE*(i+2)))
        .collect::<Vec<JoinHandle<()>>>();

    let consumer = spawn(move || {
        for _ in 0..(BATCH_SIZE*num_producers) {
            let number = rx.recv().unwrap();
            assert!(number >= BATCH_SIZE);
            assert!(number <  BATCH_SIZE*(num_producers+1));
        }
        println!("Consumer done");
    });

    for producer in producers {
        assert!(producer.join().is_ok());
    }
    assert!(consumer.join().is_ok());
}

fn run_queue(num_producers: u64) {
    fn start_producer(queue: Arc<LCRQ>, start: u64, end: u64) -> JoinHandle<()> {
        spawn(move || {
            for i in start..end {
                queue.enqueue(i);
            }
            println!("Producer {}-{} done", start, end);
        })
    }

    let lcrq = Arc::new(LCRQ::new());

    let producers = (0..num_producers)
        .map(|i| start_producer(lcrq.clone(), BATCH_SIZE*(i+1), BATCH_SIZE*(i+2)))
        .collect::<Vec<JoinHandle<()>>>();

    let cons_lcrq = lcrq.clone();
    let consumer = spawn(move || {
        for _ in 0..(BATCH_SIZE*num_producers) {
            loop {
                match cons_lcrq.dequeue() {
                    Some(number) => {
                        assert!(number >= BATCH_SIZE);
                        assert!(number <  BATCH_SIZE*(num_producers+1));
                        break;
                    },
                    None => { /* spin */ },
                }
            }
        }
        println!("Consumer done");
    });

    for producer in producers {
        assert!(producer.join().is_ok());
    }
    assert!(consumer.join().is_ok());
}

fn print_usage() {
    let app_name = env::args().nth(0).unwrap_or(String::from("speed_test"));
    println!("Usage: {} <method> [<producers>]", app_name);
    println!("Where <method> is  channel  to test with sync::mpsc::channel");
    println!("               or  queue    to test with the concurrent channel");
    println!("  and [<producers>] is the optional number of producers to use");
}

fn main() {
    match env::args().nth(1) {
        None => {
            print_usage();
        },
        Some(command) => {
            let producers = env::args().nth(2).and_then(|s| s.parse::<u64>().ok()).or(Some(2)).unwrap();
            match command.as_ref() {
                "channel" => run_channels(producers),
                "queue"   => run_queue(producers),
                _         => print_usage()
            }
        }
    }
}

