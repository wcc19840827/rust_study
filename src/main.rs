use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc, RwLock};
// extern crate rand;
// use rand::Rng;

fn main() {
    if false {
        test_1();
        test_2();
        test_3();
        test_4();
        test_5();
        test_6();
        test_7();
        test_thread();  //多线程 spawn
        test_thread1(); //线程间同步与互斥,使用同步锁 Arc::new(RwLock::new(0)
        test_thread2(); //generate_tree_r_last
    }

    test_thread3(); //线程间使用频道往主线程的vector写入数据  mpsc::sync_channel
}

fn test_thread3() {
    println!("start");

    let mut gpu_busy_flag = Vec::new();
    let (faulty_tx, faulty_rx) = mpsc::sync_channel(20);

    rayon::scope(|s| {
        for i in 0..20 {
            let thread_faulty_tx = faulty_tx.clone();

            s.spawn(move |_| {
                thread_faulty_tx.send(i).unwrap();
            });
        }

    }); // scope_end
    drop(faulty_tx);

    for received in faulty_rx.iter() {
        // let received = faulty_rx.recv().unwrap();
        println!("------wcc receive faulty {}", received);
        gpu_busy_flag.push(received)
    }

    println!("end");
}

fn test_thread2() {//generate_tree_r_last
    println!("start");

    let _bus_num = 4;
    let mut gpu_busy_flag = Vec::new();
    // TODO-Ryan:
    // gpu_busy_flag.resize(_bus_num, Arc::new(RwLock::new(0)));
    for _ in 0.._bus_num {
        gpu_busy_flag.push( Arc::new(RwLock::new(0)) )
    }

    let config_count = 8;
    rayon::scope(|s| {
        for i in (0..config_count).step_by(_bus_num) {
            for j in 0.._bus_num {

                let i = i + j;
                if i == config_count {
                    break;
                }

                let (builder_tx, builder_rx) = mpsc::sync_channel(0);
                // let (builder_tx, builder_rx) = mpsc::sync_channel::<(Vec<u16>, bool)>(0);

                s.spawn(move |_| {

                    println!("[tree_r_last] encoded");

                    builder_tx
                    .send((1, false))
                    .expect("failed to send encoded");

                    println!("[tree_r_last] builder_tx");
                });

                // let mut gpu_busy_flag = gpu_busy_flag.clone();
                let gpu_busy_flag = gpu_busy_flag.clone();
                s.spawn(move |_| {
                    println!("[tree_r_last] 1");
                    let (_encoded, _is_final) =
                                    builder_rx.recv().expect("failed to recv encoded data");
                    
                    println!("[tree_r_last] 2");
                    // find_idle_gpu
                    println!("[tree_c] begin to find idle gpu i={}, j={}", i, j);
                    let mut find_idle_gpu: i32 = -1;
                    loop {
                        for i in 0.._bus_num {
                            if *gpu_busy_flag[i].read().unwrap() == 0 {
                                *gpu_busy_flag[i].write().unwrap() = 1;
                                find_idle_gpu = i as i32;

                                println!("[tree_c] find_idle_gpu={} i={}, j={}", find_idle_gpu, i, j);// TODO-Ryan:
                                break;
                            }
                        }

                        if find_idle_gpu == -1 {
                            thread::sleep(Duration::from_millis(1));
                        }else{
                            break;
                        }
                    }

                    assert!(find_idle_gpu>=0);
                    let find_idle_gpu: usize = find_idle_gpu as usize;
                    println!("[tree_c] Use multi GPUs, total_gpu={}, i={}, use_gpu_index={}", _bus_num, i, find_idle_gpu);

                    // let factor = rand::thread_rng().gen_range(1, 10); //默认使用 i32
                    // thread::sleep(Duration::from_millis(factor*1000));

                    *gpu_busy_flag[find_idle_gpu].write().unwrap() = 0; 
                    println!("[tree_c] set gpu idle={} i={}, j={}", find_idle_gpu, i, j);// TODO-Ryan:

                    // thread::sleep(Duration::from_millis(factor*1000));
                });
            }
        }

    }); // scope_end

    println!("end");
}

fn test_thread1() {
    println!("start");

    let _bus_num = 4;
    let mut gpu_busy_flag = Vec::new();
    // TODO-Ryan:
    // gpu_busy_flag.resize(_bus_num, Arc::new(RwLock::new(0)));
    for _ in 0.._bus_num {
        gpu_busy_flag.push( Arc::new(RwLock::new(0)) )
    }

    let config_count = 8;
    rayon::scope(|s| {
        for i in (0..config_count).step_by(_bus_num) {
            for j in 0.._bus_num {

                let i = i + j;
                if i == config_count {
                    break;
                }

                // let mut gpu_busy_flag = gpu_busy_flag.clone();
                let gpu_busy_flag = gpu_busy_flag.clone();
                s.spawn(move |_| {

                    // find_idle_gpu
                    println!("[tree_c] begin to find idle gpu i={}, j={}", i, j);
                    let mut find_idle_gpu: i32 = -1;
                    loop {
                        for i in 0.._bus_num {
                            if *gpu_busy_flag[i].read().unwrap() == 0 {
                                *gpu_busy_flag[i].write().unwrap() = 1;
                                find_idle_gpu = i as i32;

                                println!("[tree_c] find_idle_gpu={} i={}, j={}", find_idle_gpu, i, j);// TODO-Ryan:
                                break;
                            }
                        }

                        if find_idle_gpu == -1 {
                            thread::sleep(Duration::from_millis(1));
                        }else{
                            break;
                        }
                    }

                    assert!(find_idle_gpu>=0);
                    let find_idle_gpu: usize = find_idle_gpu as usize;
                    println!("[tree_c] Use multi GPUs, total_gpu={}, i={}, use_gpu_index={}", _bus_num, i, find_idle_gpu);

                    // let factor = rand::thread_rng().gen_range(1, 10); //默认使用 i32
                    // thread::sleep(Duration::from_millis(factor*1000));

                    *gpu_busy_flag[find_idle_gpu].write().unwrap() = 0; 
                    println!("[tree_c] set gpu idle={} i={}, j={}", find_idle_gpu, i, j);// TODO-Ryan:

                    // thread::sleep(Duration::from_millis(factor*1000));
                });
            }
        }

    }); // scope_end

    println!("end");
}

fn test_thread() {
    const CONST: i32 = 1000;

    let gloab = Arc::new(RwLock::new(0));

    let gloab_clone = gloab.clone();
    let thread1 = thread::spawn(move || {
        // {
            //获取写锁的变量出了作用域, 解锁
            let mut _data = gloab_clone.write().unwrap();

            // 获取读所这行执行完就解锁了
            let mut _data = gloab_clone.read().unwrap();
        // }
            for _ in 0..CONST {
                print!("{} ",'A');
            }
        // }
    });

    let gloab_clone = gloab.clone();
    let thread2 = thread::spawn(move || {
        // {
            let mut _data = gloab_clone.write().unwrap();
            // let mut _data = gloab_clone.read().unwrap();
        // }
            for _ in 0..CONST {
                print!("{} ",'*');
            }
        // }
    });

    thread1.join().ok();
    thread2.join().ok();

    println!("{}", gloab.write().unwrap());
}

fn test_1(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn test_2(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { //在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权
        println!("Here's a vector: {:?}", v);

        drop(v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();
}

// 将一个值传送到通道中，将无法再使用这个值
fn test_3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); //通道中发送完 val 值 之后, 不能再使用它
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 通过克隆发送者来创建多个生产者 (mpsc 是 multiple producer, single consumer 的缩写)
fn test_4() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// 互斥器 Mutex
fn test_5() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();    //这个调用会阻塞当前线程，直到我们拥有锁为止。
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Rust的可变不共享,共享不可变的原则. 要在线程间共享可变数据, 需要用到 Arc+Mutex 或者 Arc+RwLock

// 原子引用计数 Arc<T>,   使用 Arc<T> 包装一个 Mutex<T> 能够实现在多线程之间共享所有权
fn test_6() {
    let counter = Arc::new(Mutex::new(0)); // Mutex<T> 是一个智能指针
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn test_7() {
    const CONST: i32 = 10000;

    let gloab = Arc::new(RwLock::new(0));

    let gloab_clone = gloab.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..CONST {
            *gloab_clone.write().unwrap() += 1;
            print!("{} ",'A');
        }
    });

    let gloab_clone = gloab.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..CONST {
            *gloab_clone.write().unwrap() -= 1;
            print!("{} ",'B');
        }
    });

    thread1.join().ok();
    thread2.join().ok();

    println!("{}", gloab.write().unwrap());
}