pub fn option_function_first_some(input: Option<String>) -> Option<String> {
    let input = match input {
        Some(input) => input,
        None => return None,
    };
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_after_fix(input: Option<String>) -> Option<String> {
    let input = match input {
        Some(input) => input,
        none => return none,
    };
    Some(input.to_ascii_lowercase())
}

pub fn result_function_first_ok(input: Result<String, String>) -> Result<String, String> {
    let input = match input {
        Ok(input) => input,
        Err(err) => return Err(err),
    };
    Ok(input.to_ascii_lowercase())
}

pub fn result_function_first_ok_after_fix(input: Result<String, String>) -> Result<String, String> {
    let input = match input {
        Ok(input) => input,
        err => return err,
    };
    Ok(input.to_ascii_lowercase())
}

pub struct Struct {
    field: String,
}

pub fn struct_function(input: Option<Struct>) -> Option<Struct> {
    let input = match input {
        None => return None,
        Some(Struct { field }) => Some(Struct { field })
    };
    input.or_else(|| Some(Struct { field: "".to_string() }))
}

#[allow(clippy::needless_match)]
pub fn struct_function_after_fix(input: Option<Struct>) -> Option<Struct> {
    let input = match input {
        None => return None,
        some => some
    };
    input.or_else(|| Some(Struct { field: "".to_string() }))
}

// TODO: Enum function
