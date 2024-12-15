use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    basic_async_await("yo".to_string()).await;
    futures_are_lazy().await;
    morph_data_across_threads().await;





}


// how to use basic async/await
async fn basic_async_await(msg: String) {
   println!("I am an async function! Here is my message: {}", msg); 
}

// how to get a future, and await it later
async fn futures_are_lazy() {
    let fut = basic_async_await("lazyyyy".to_string());
    fut.await;
}

// how to morph data across multiple threads
async fn morph_data_across_threads() {
    let str = String::from("I will be morphed!");
    let str_arc = Arc::new(Mutex::new(str));
    let mut handles = vec![];
    for i in 0..10 {
        let str_clone = Arc::clone(&str_arc);
        let task = tokio::spawn(async move {
            // lock the mutex to modify it
            let mut val = str_clone.lock().await;
            val.push_str(&i.to_string());
        });
        handles.push(task);
    }
    for task in handles {
        task.await.unwrap();
    }
    // retrieve the final value as str is expended
    let final_str = str_arc.lock().await;
    println!("{}", final_str);
}




