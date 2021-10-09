use futures::stream::{self, StreamExt};
use std::future;
use std::future::Future;

pub(crate) async fn use_async_block(s: String) {
    async {
        println!("{}", s.clone());
    }
    .await;

    println!("{}", s);
}

pub(crate) async fn use_async_block_in_stream(s: String) {
    stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        .for_each(|v| async {
            let v = v;
            println!("{}", s.clone());
            println!("{}", v);
        })
        .await;

    println!("{}", s);
}

pub(crate) async fn use_future_in_stream(s: String) {
    stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        .for_each(|v| {
            println!("{}", s.clone());
            println!("{}", v);
            future::ready(())
        })
        .await;

    println!("{}", s);
}

pub(crate) fn use_async_block_in_stream_return(s: String) -> impl Future<Output = ()> {
    async move {
        stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
            .for_each(|v| async {
                let v = v;
                println!("{}", s.clone());
                println!("{}", v);
            })
            .await
    }
}

pub(crate) fn use_future_in_stream_return(s: String) -> impl Future<Output = ()> {
    stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()]).for_each(move |v| {
        println!("{}", s.clone());
        println!("{}", v);
        future::ready(())
    })
}
