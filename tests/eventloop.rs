extern crate aers;

use aers::EventLoop;

#[test]
fn create() {
    EventLoop::create(3);
}

#[test]
fn stop_timed() {
    let mut el = EventLoop::create(3);
    let b = Box::new(|| {
        println!("1");
    });
    el.create_time_event(1, &*b);
    el.main();
}
