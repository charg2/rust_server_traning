use crate::executor::IJob;

pub mod lock_queue;
pub mod executor;

fn main()
{
    println!("hello world0");


    let mut executor = executor::Executor::new();
    executor.post( move ||
        {
            println!("hello world1");
        });

    executor.post( move ||
        {
            println!("hello world2");
        });

    executor.post( move ||
        {
            println!("hello world3");
        });

    executor.execute();
}