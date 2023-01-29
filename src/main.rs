use crate::async_await::{
    use_async_block, use_async_block_in_stream, use_async_block_in_stream_return,
    use_future_in_stream, use_future_in_stream_return,
};
use crate::closure::{
    append_string, append_string_output_trait, equals_hello, equals_hello_input_trait,
};
use crate::function::{use_function, use_function_async};

mod async_await;
mod clone;
mod closure;
mod collect;
mod function;
mod scope;

#[tokio::main]
async fn main() {
    println!("{}", equals_hello()("hello".to_string()));
    println!("{}", equals_hello_input_trait()(Box::new("hello")));
    println!("{}", append_string()("hello".to_string()));
    println!(
        "{}",
        append_string_output_trait()("hello".to_string()).to_string()
    );

    use_async_block("async".to_string()).await;
    use_async_block_in_stream("async".to_string()).await;
    use_future_in_stream("future".to_string()).await;
    use_async_block_in_stream_return("async".to_string()).await;
    use_future_in_stream_return("future".to_string()).await;

    use_function("input".to_string()).await;
    use_function_async("input".to_string()).await;

    scope::scope_fixed();
    scope::scope_expanded();
    scope::scope();

    collect::collect_example();

    clone::function();
}
