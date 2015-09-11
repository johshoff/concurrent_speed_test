Compare the speed of my [concurrent queue implementation](https://github.com/johshoff/concurrent_queue) against Rust's `mpsc::channel`.

Building
--------

Using **nightly** rust, run

	cargo build --release

Running
-------

To test using `mpsc::channel`, run

	time target/release/speed-test channel

To test using the concurrent queue, run

	time target/release/speed-test queue

