#![feature(unsafe_destructor)]
#![feature(libc)]
extern crate libc;

use std::ptr;

#[link(name = "ae")]

extern {
    fn aeCreateEventLoop(setsize: libc::c_int) -> *mut libc::c_void;
    fn aeDeleteEventLoop(el: *mut libc::c_void);
    fn aeStop(el: *mut libc::c_void);
    fn aeCreateFileEvent(
            el: *mut libc::c_void,
            fd: libc::c_int,
            mask: libc::c_int,
            proc_: extern fn(
                el: *mut libc::c_void,
                fd: libc::c_int,
                clientData: *mut libc::c_void,
                mask: libc::c_int
                ),
            clientData: *mut libc::c_void
            ) -> libc::c_int;
    fn aeDeleteFileEvent(
            el: *mut libc::c_void,
            fd: libc::c_int,
            mask: libc::c_int
            );
    fn aeGetFileEvents(
            el: *mut libc::c_void,
            fd: libc::c_int
            ) -> libc::c_int;
    fn aeCreateTimeEvent(
            el: *mut libc::c_void,
            milliseconds: libc::c_longlong,
            proc_: extern fn(
                el: *mut libc::c_void,
                id: libc::c_longlong,
                clientData: *mut libc::c_void
                ),
            clientData: *mut libc::c_void,
            finalizerProc: extern fn(
                el: *mut libc::c_void,
                clientData: *mut libc::c_void
                )
            ) -> libc::c_longlong;
    fn aeDeleteTimeEvent(
            el: *mut libc::c_void,
            id: libc::c_longlong
            ) -> libc::c_int;
    fn aeProcessEvents(
            el: *mut libc::c_void,
            flags: libc::c_int
            ) -> libc::c_int;
    fn aeWait(
            fd: libc::c_int,
            mask: libc::c_int,
            milliseconds: libc::c_longlong
            ) -> libc::c_int;
    fn aeMain(el: *mut libc::c_void);
    fn aeGetApiName() -> std::ffi::CString;
    fn aeSetBeforeSleepProc(
            el: *mut libc::c_void,
            beforesleep: extern fn(el: *mut libc::c_void)
            );
    fn aeGetSetSize(el: *mut libc::c_void) -> libc::c_int;
    fn aeResizeSetSize(
            el: *mut libc::c_void,
            setsize: libc::c_int
            ) -> libc::c_int;
}

pub struct EventLoop {
    el: *mut libc::c_void
}

impl EventLoop {
    pub fn create(setsize: usize) -> EventLoop {
        unsafe {
            return EventLoop {
                el: aeCreateEventLoop(setsize as libc::c_int)
            }
        }
    }
}
impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe {
            aeDeleteEventLoop(self.el);
        }
    }
}
