use futures::{self, Future};
use std::{collections::LinkedList, io, thread, time};

fn menu_str() -> String {
    String::from(
        "
    1) print ten AAA each sec;
    2) print five BBB each two sec;
    0) exit;
    ",
    )
}

async fn ask_user() -> i32 {
    loop {
        print!("Select variant: {}", menu_str());
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("can't read user input");
        match answer.trim().parse() {
            Ok(answer) if answer >= 0 && answer <= 2 => return answer,
            _ => {
                println!("Bad input, again: \n");
                continue;
            }
        }
    }
}

async fn print_aaa() {
    for i in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
        println!("\tAAA {}", i);
    }
}

async fn print_bbb() {
    for i in 0..5 {
        thread::sleep(time::Duration::from_secs(1));
        println!("\tBBB {}", i);
    }
}

async fn do_nothing() {
    println!("Do nothing called");
}

type ActionFuture = Box<dyn Future<Output=(())>>;

fn answer_to_action(answer: i32) -> ActionFuture {
    match answer {
        1 => Box::new(print_aaa()),
        2 => Box::new(print_bbb()),
        _ => Box::new(print_bbb()),
    }
}

async fn run_loop() {
    let mut q = LinkedList::<ActionFuture>::new();


    loop {
        let answer = ask_user().await;
        if answer == 0 {
            break;
        }



    }
    println!("Work done");
}

fn main() {
    futures::executor::block_on(run_loop());
}
