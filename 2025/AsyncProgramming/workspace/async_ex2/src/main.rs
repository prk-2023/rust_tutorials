use std::time::Duration;

// Message passing between futures:
fn main() {
    println!("1. Self messaging ");
    msg_send_recv_insameblock();
    println!("2. multi messaging to demo: async block executes linearly ");
    multi_msg();
    println!("3. Multi message between different async block");
    multi_msg_sepearatea_async_blocks();
    println!("4. Multi message between multi async block");
    multi_msg_multi_async_blocks();
}

fn msg_send_recv_insameblock() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("Hi from task1");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received: '{received}'");
    });
}

fn multi_msg() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];
        for val in vals {
            println!("Sending: {}", val);
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
        //send a termination message
        println!("Sending msg to terminate");
        trpl::sleep(Duration::from_millis(3000)).await;
        let _ = tx.send("close".to_string());

        while let Some(value) = rx.recv().await {
            if value == "close" {
                break;
            } else {
                println!("received '{value}'");
            }
        }
    });
}

fn multi_msg_sepearatea_async_blocks() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        //move forces async block to take ownership of the var it uses from its env.
        //this ensures the sender is dropped after sending the last message allowing prog to
        //terminate.
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    });
}

fn multi_msg_multi_async_blocks() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        //async channel support multi producers and we can clone the sender to allow multi async
        //blocks to send messages
        let tx1 = tx.clone();
        //move forces async block to take ownership of the var it uses from its env.
        //this ensures the sender is dropped after sending the last message allowing prog to
        //terminate.
        let tx1_fut = async move {
            let vals = vec![
                String::from("Clone:hi"),
                String::from("Clone:from"),
                String::from("Clone:the"),
                String::from("Clone:future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
