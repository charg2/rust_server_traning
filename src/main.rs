use crate::executor::IJob;
use tokio::runtime::Runtime;
use std::sync::Arc;

pub mod lock_queue;
pub mod executor;

#[tokio::main]
async fn main()
{
    println!("hello world0");

    let executor = Arc::new(executor::Executor::new());
    executor.as_ref().post( move ||
        {
            println!("hello world1");
        });

    executor.as_ref().post( move ||
        {
            println!("hello world2");
        });

    executor.as_ref().post( move ||
        {
            println!("hello world3");
        });

    //executor.execute();
}