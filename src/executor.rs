use std::ptr::null;
use crate::executor;
use crate::lock_queue::LockQueue;

pub trait IJob
{
    fn execute(&mut self);
}

pub struct Job
{
    func: Option<Box<dyn FnOnce() + Send + 'static>>,
}

pub struct Executor
{
    job_queue: LockQueue<Job>
}

impl IJob for Executor
{
    fn execute(&mut self)
    {
        loop
        {
            match self.job_queue.try_pop()
            {
                Some(mut job) =>
                {
                    job.execute();
                }
                None =>
                {
                    return;
                }
            }
        }
    }
}

impl IJob for Job
{
    fn execute(&mut self)
    {
        if let Some(func) = self.func.take()
        {
            func();
        }
    }
}

impl Job
{
    pub fn new<F>(func: F) -> Job
    where
        F: FnOnce() + Send + 'static,
    {
        Job {
            func: Some(Box::new(func)),
        }
    }
}

impl Executor
{
    pub fn new() -> Executor
    {
        Executor { job_queue: LockQueue::new(), }
    }

    pub fn post<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.job_queue.push( executor::Job::new(job) );
    }
}