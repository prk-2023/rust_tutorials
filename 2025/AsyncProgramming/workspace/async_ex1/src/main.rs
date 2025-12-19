use std::time::Duration;

fn main() {
    println!("Spawn two async tasks, [quite as soon as one task completes] ");
    do_async_nosync();
    println!("Spawn two async tasks, [ensure all tasks complete before return] ");
    do_async_sync();
    println!("use trpl::join multiple futures into a single futures block");
    do_join_multi_futures();
}
//-------------------------------------------------------------------------
fn do_async_nosync() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
            println!("First task complete!");
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        println!("Second task complete!");
    });
}
/*
- main function is useped with block_on : which initializes a runtime to execute the future passed
- There are two loops :
    - first future : that is spawn with `spawn_task`
    - second future directly from main function  loop
*/

//-------------------------------------------------------------------------
fn do_async_sync() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
            println!("First task complete!");
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
            println!("Second task complete!");
        }
        handle.await.unwrap();
    });
}
/*
- using the join handle used to await the completion of the first task.
*/
//-------------------------------------------------------------------------

fn do_join_multi_futures() {
    trpl::block_on(async {
        let future1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let future2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(future1, future2).await;
    });
}
