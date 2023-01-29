use crate::async_await::{
    use_async_block, use_async_block_in_stream, use_async_block_in_stream_return,
    use_future_in_stream, use_future_in_stream_return,
};
use crate::closure::{
    append_string, append_string_output_trait, equals_hello, equals_hello_input_trait,
};
use crate::function::{use_function, use_function_async};

mod async_await;
mod clippy;
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

    clippy::redundant_clone::function();
    clippy::redundant_clone::function_after_fix();

    clippy::question_mark::option_function_first_none(None);
    clippy::question_mark::option_function_first_none_after_fix(None);
    clippy::question_mark::option_function_first_some(None);
    clippy::question_mark::option_function_first_some_after_fix(None);
    clippy::question_mark::option_function_first_some_no_match_components(None);
    clippy::question_mark::option_function_first_some_no_match_components_after_fix(None);
    clippy::question_mark::result_function_first_ok(Ok("".to_string())).unwrap();
    clippy::question_mark::result_function_first_ok_after_fix(Ok("".to_string())).unwrap();
    clippy::question_mark::result_function_first_ok_no_match_components(Ok("".to_string()))
        .unwrap();
    clippy::question_mark::result_function_first_ok_no_match_components_after_fix(Ok(
        "".to_string()
    ))
    .unwrap();
    clippy::question_mark::result_function_first_err(Ok("".to_string())).unwrap();
    clippy::question_mark::result_function_first_err_after_fix(Ok("".to_string())).unwrap();

    clippy::unnecessary_match_components::option_function_first_some(None);
    clippy::unnecessary_match_components::option_function_first_some_after_fix(None);
    clippy::unnecessary_match_components::result_function_first_ok(Ok("".to_string())).unwrap();
    clippy::unnecessary_match_components::result_function_first_ok_after_fix(Ok("".to_string()))
        .unwrap();
    clippy::unnecessary_match_components::struct_function(None);
    clippy::unnecessary_match_components::struct_function_after_fix(None);
}
