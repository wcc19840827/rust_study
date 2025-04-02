use tokio::time::{sleep, Duration};

//基本使用：异步任务
async fn test1() {
    println!("任务开始...");
    sleep(Duration::from_secs(2)).await;
    println!("任务结束！");
}

//--------------------------------------------------------------------

//并发任务（tokio::spawn）
async fn test2() {
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("任务 1 完成");
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("任务 2 完成");
    });

    // 等待两个任务完成
    task1.await.unwrap();
    task2.await.unwrap();
}

//--------------------------------------------------------------------

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// 异步 TCP 服务器
async fn test3() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器监听 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("收到连接: {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    println!("收到数据: {}", String::from_utf8_lossy(&buffer[..n]));
                    let _ = socket.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, Tokio!\n").await;
                }
                _ => println!("连接关闭"),
            }
        });
    }
}

//--------------------------------------------------------------------

use tokio::time::{interval};

//定时任务（tokio::time）
async fn test4() {
    let mut timer = interval(Duration::from_secs(1));

    for _ in 0..5 {
        timer.tick().await; //每隔1秒
        println!("定时任务执行！");
    }
}

//--------------------------------------------------------------------

use std::sync::Arc;
use tokio::sync::Mutex;

// 共享数据访问（tokio::sync::Mutex）
async fn test5() {
    let counter = Arc::new(Mutex::new(0)); //安全的共享数据访问

    let c1 = Arc::clone(&counter);
    let task1 = tokio::spawn(async move {
        let mut num = c1.lock().await;
        *num += 1;
    });

    let c2 = Arc::clone(&counter);
    let task2 = tokio::spawn(async move {
        let mut num = c2.lock().await;
        *num += 1;
    });

    task1.await.unwrap();
    task2.await.unwrap();

    println!("最终计数器值: {}", *counter.lock().await); //2
}

//--------------------------------------------------------------------

use tokio::sync::mpsc;

//任务通道（tokio::sync::mpsc）
//Tokio 提供 mpsc（多生产者单消费者）通道，实现 异步消息传递：

// #[tokio::main]   // 入口函数，自动初始化 Tokio 运行时
async fn test6() {
    let (tx, mut rx) = mpsc::channel(32); //32表示缓冲区大小, 缓冲区满,则会阻塞生产者

    tokio::spawn(async move {
        tx.send("Hello from Tokio!").await.unwrap();
        
        // if let Err(e) = solution_sender_clone.try_send(solution_tuple) 
    });

    while let Some(msg) = rx.recv().await {
        println!("收到消息: {}", msg);
    }
}

//--------------------------------------------------------------------

//默认情况下，Tokio 运行时使用多线程调度任务
#[tokio::main(flavor = "multi_thread", worker_threads = 4)] //指定线程数
async fn main() {
    println!("Tokio 多线程运行时启动！");

    test6().await; // 调用异步函数
}

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     println!("Tokio 单线程运行时启动！");
// }
