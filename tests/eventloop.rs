extern crate aers;

use aers::EventLoop;

#[test]
fn create() {
    EventLoop::create(3);
}
