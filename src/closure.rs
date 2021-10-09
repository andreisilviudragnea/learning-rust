pub(crate) fn equals_hello() -> impl Fn(String) -> bool {
    |string| string == "hello"
}

pub(crate) fn equals_hello_input_trait() -> impl Fn(Box<dyn ToString>) -> bool {
    |string| string.to_string() == "hello"
}

pub(crate) fn append_string() -> impl Fn(String) -> String {
    |string| string + "Suffix"
}

pub(crate) fn append_string_output_trait() -> impl Fn(String) -> Box<dyn ToString> {
    |string| Box::new(string + "Suffix")
}
