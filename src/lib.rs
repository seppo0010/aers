#![feature(unsafe_destructor)]
#![feature(libc)]
#![feature(core)]
extern crate libc;

#[link(name = "ae")]

extern {
    #![allow(dead_code)]
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
                client_data: *const Fn(),
                mask: libc::c_int
                ),
            client_data: *const Fn(),
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
                client_data: *const Fn()
                ),
            client_data: *const Fn(),
            finalizerProc: extern fn(
                el: *mut libc::c_void,
                client_data: *const Fn()
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
    el: *mut libc::c_void,
}

extern fn time_event(el: *mut libc::c_void, id: libc::c_longlong, client_data: *const Fn()) {
    unsafe { (*client_data)(); }
}

extern fn time_finalizer(el: *mut libc::c_void, client_data: *const Fn()) {
}

impl EventLoop {
    pub fn create(setsize: usize) -> EventLoop {
        unsafe {
            return EventLoop {
                el: aeCreateEventLoop(setsize as libc::c_int),
            }
        }
    }

    pub fn create_time_event<'a>(
            &mut self,
            milliseconds: i64,
            proc_: *const Fn()) {
        unsafe {
            let _ = aeCreateTimeEvent(
                    self.el,
                    milliseconds as libc::c_longlong,
                    time_event,
                    proc_,
                    time_finalizer,
                    );
        }
    }

    pub fn main(&self) {
        unsafe {
            aeMain(self.el);
        }
    }

    pub fn stop(&self) {
        unsafe {
            aeStop(self.el);
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
