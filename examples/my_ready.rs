use std::{pin::Pin, task};

use futures::Future;

// my_ready! = Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(val),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

#[tokio::main]
async fn main() {
    // let mut cx = task::Context::from_waker(futures::task::noop_waker_ref());
    // let ret = poll_fut(&mut cx);
    let fut = MyFut::new(42);
    let ret = fut.await;
    println!("ret: {}", ret);
    println!("ret: {}", ret);
}

#[allow(unused)]
fn poll_fut(cx: &mut std::task::Context<'_>) -> std::task::Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        if self.polled {
            task::Poll::Ready(self.v)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            task::Poll::Pending
        }
    }
}
