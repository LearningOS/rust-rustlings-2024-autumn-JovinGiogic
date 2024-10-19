// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(
            thread::spawn(move || {
                let start = Instant::now();  // 获取当前时间
                thread::sleep(Duration::from_millis(250));
                println!("thread {} is complete", i);
                start.elapsed().as_millis()  // elapsed()用来计算从 start 开始到当前时间所经过的时间，as_millis()转化为毫秒
            })  // 返回的是JoinHandle<u128>
        );  // 依次push到handles中
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());  // join表示等待该线程结束再继续，会返回Result，用unwarp获取Ok值（即运行时间），收集到results
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
