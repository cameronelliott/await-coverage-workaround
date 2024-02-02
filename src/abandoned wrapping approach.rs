use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::mem::replace;

struct Join<A, B> {
    a: Option<A>,
    b: Option<B>,
}

impl<A, B> Join<A, B> {
    fn new(a: A, b: B) -> Self {
        Join { a: Some(a), b: Some(b) }
    }
}

impl<A, B> Future for Join<A, B>
where
    A: Future<Output = ()>,
    B: Future<Output = ()>,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut a_done = false;
        let mut b_done = false;

        if let Some(a) = &mut self.a {
            let a_poll = Pin::new(a).poll(cx);
            if a_poll.is_ready() {
                a_done = true;
            } else if a_poll.is_pending() {
                replace(&mut self.a, None);
            }
        }

        if let Some(b) = &mut self.b {
            let b_poll = Pin::new(b).poll(cx);
            if b_poll.is_ready() {
                b_done = true;
            } else if b_poll.is_pending() {
                replace(&mut self.b, None);
            }
        }

        if a_done && b_done {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let task_a = async {
        // Perform some asynchronous task A
        println!("Task A started");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        println!("Task A completed");
    };

    let task_b = async {
        // Perform some asynchronous task B
        println!("Task B started");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Task B completed");
    };

    let join_task = Join::new(task_a, task_b);
    join_task.await;
}
