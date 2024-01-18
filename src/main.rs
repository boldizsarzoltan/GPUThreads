use std::ffi::c_void;
use std::{ptr, thread};
use std::collections::HashMap;
use std::ptr::null;
use std::sync::{Arc, Mutex, Condvar};
use libc::c_int;

const PAUSE_TIME: u64 = 300;

extern "C" fn thread_function(data_ptr: *mut c_void) -> *mut c_void
{
    let mut data: Box<ThreadData> = unsafe { Box::from_raw(data_ptr as *mut ThreadData) };
    let mut local_counter = 0;
    println!("Thread {} started, {}", data.thread_number, local_counter);

    let mut pause_flag = data.pause_flag.lock().unwrap();
    while (*pause_flag  && local_counter < data.max_iterations) {
        println!("Thread {} paused...", data.thread_number);
        thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));
        pause_flag = data.start_var.wait(pause_flag).unwrap();
        local_counter +=1 ;
        println!("Thread {} incremented to {}", data.thread_number, local_counter);
        thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));
        if(local_counter >= data.max_iterations) {
            data.finished = Arc::from(true);
        }
    }
    println!("Thread {} is {}", data.thread_number, data.finished);
    println!("Thread {} notified main", data.thread_number);
    thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));

    println!("Thread {} exiting...", data.thread_number);

    // Returning null pointer as the thread's exit status
    ptr::null_mut()
}

#[derive(Debug)]
struct ThreadData {
    pub thread_number: i32,
    pub max_iterations: i32,
    pub pause_flag: Arc<Mutex<bool>>,
    pub start_var: Arc<Condvar>,
    pub finished: Arc<bool>
}
impl ThreadData {
    fn new(thread_number: i32, max_iterations: i32) -> Self {
        ThreadData {
            thread_number,
            pause_flag: Arc::new(Mutex::new(true)),
            start_var: Arc::new(Condvar::new()),
            finished: Arc::new(false),
            max_iterations,
        }
    }
}

impl Clone for ThreadData {
    fn clone(&self) -> Self {
        // Manually clone each field
        ThreadData {
            thread_number: self.thread_number,
            max_iterations: self.max_iterations,
            pause_flag: Arc::clone(&self.pause_flag),
            start_var: Arc::clone(&self.start_var),
            finished: Arc::clone(&self.finished),
        }
    }
}

fn main() {
    let number_of_threads:i32 = 5;
    let max_operations:i32 = 5;


    let mut thread_ids: HashMap<i32, libc::pthread_t> = HashMap::with_capacity(number_of_threads as usize);
    let mut datas: HashMap<i32, Box<ThreadData>> = HashMap::with_capacity(number_of_threads as usize);

    for i in 0..number_of_threads {
        unsafe {
            let current_thread_data = ThreadData::new(
                i,
                max_operations
            );
            let data_to_pass = Box::new(current_thread_data);
            let data_ptr = Box::into_raw(data_to_pass.clone());
            let mut thread_id: libc::pthread_t =  std::mem::zeroed();
            let result = libc::pthread_create(
                &mut thread_id,
                ptr::null(),
                thread_function,
                data_ptr as *mut c_void,
            );
            datas.insert(i, data_to_pass.clone());
            thread_ids.insert(i, thread_id);

            if result != 0 {
                println!("Failed to create thread {}. Error code: {}", i, result);
            }
            else {
                println!("Create thread {}. Error code: {}", i, result);
            }
        }
    }

    while !thread_ids.is_empty() {
        let mut i = 0;
        for threads in thread_ids.clone().keys() {
            let local_data = datas.get(&(i)).unwrap();
            println!("Main thread notfied {:?}", local_data.thread_number);
            thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));
            local_data.start_var.notify_one();
            let mut pause_flag = local_data.pause_flag.lock().unwrap();
            println!("Main thread returned {:?}", local_data.thread_number);
            thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));
            println!("Thread {} {:?}", threads, local_data);
            if(*local_data.finished) {
                println!("Thread {:?} finished", threads);
                thread_ids.remove(&threads);
                println!("Remaining {:?}", thread_ids);
                thread::sleep(std::time::Duration::from_millis(PAUSE_TIME));
            }
            i += 1;
        }
    }

    // println!("Thread create:{:?}", thread);
    // println!("Thread join:{:?}", unsafe { pthread_join(thread_id, ptr::null_mut()) });
}