use std::ffi::c_void;
use std::ptr;
use std::ptr::null;
use libc::{c_int, c_ulong, pthread_attr_t, pthread_create, pthread_join, pthread_t};

extern "C" fn thread_function(data_ptr: *mut c_void) -> *mut c_void
{
    let data: Box<ThreadData> = unsafe { Box::from_raw(data_ptr as *mut ThreadData) };
    println!("hi {:?}", data);
    ptr::null_mut()
}

#[derive(Debug)]
struct ThreadData {
    pub thread_number: i32,
    pub max_iterations: i32,
}

fn main() {
    let _number_of_threads = 5;
    let _max_operations = 5;

    let data_to_pass = Box::new(ThreadData {
        thread_number: 1,
        max_iterations: 5,
    });
    let data_ptr = Box::into_raw(data_to_pass);

    let mut thread_id: libc::pthread_t = unsafe { std::mem::zeroed() };
    let thread = unsafe {
        pthread_create(
            &mut thread_id,
            null(),
            thread_function ,
            data_ptr as *mut c_void,
        )
    };

    println!("Thread create:{:?}", thread);
    println!("Thread join:{:?}", unsafe { pthread_join(thread_id, ptr::null_mut()) });
}