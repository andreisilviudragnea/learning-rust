use futures::stream::{self, StreamExt};
use std::future::Future;

fn use_borrow(str: &str, input: &str) -> impl Future<Output = ()> {
    println!("{str} {input}");
    std::future::ready(())
}

pub(crate) async fn use_function(input: String) {
    stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        .for_each(|v| use_borrow(&v, &input))
        .await;
}

async fn use_borrow_async(str: &str, input: &str) {
    println!("{str} {input}");
}

pub(crate) async fn use_function_async(input: String) {
    stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        .for_each(|v| async {
            let v = v;
            use_borrow_async(&v, &input).await;
        })
        .await;
}
