pub fn option_function_first_none(input: Option<String>) -> Option<String> {
    let input = match input {
        None => return None,
        Some(input) => input,
    };
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_none_after_fix(input: Option<String>) -> Option<String> {
    let input = input?;
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_none_no_return(input: Option<String>) -> Option<String> {
    match input {
        None => None,
        Some(input) => Some(input.to_ascii_lowercase())
    }
}

pub fn option_function_first_none_no_return_after_fix(input: Option<String>) -> Option<String> {
    let input = input?;
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some(input: Option<String>) -> Option<String> {
    let input = match input {
        Some(input) => input,
        None => return None,
    };
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_after_fix(input: Option<String>) -> Option<String> {
    let input = input?;
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_no_return(input: Option<String>) -> Option<String> {
    match input {
        Some(input) => Some(input.to_ascii_lowercase()),
        None => None,
    }
}

pub fn option_function_first_some_no_return_after_fix(input: Option<String>) -> Option<String> {
    let input = input?;
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_no_match_components(input: Option<String>) -> Option<String> {
    let input = match input {
        Some(input) => input,
        none => return none,
    };
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_no_match_components_after_fix(
    input: Option<String>,
) -> Option<String> {
    let input = input?;
    Some(input.to_ascii_lowercase())
}

pub fn option_function_first_some_no_match_components_no_return(input: Option<String>) -> Option<String> {
    match input {
        Some(input) => Some(input.to_ascii_lowercase()),
        none => none,
    }
}

pub fn option_function_first_some_no_match_components_no_return_after_fix(input: Option<String>) -> Option<String> {
    let input = input?;
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
    let input = input?;
    Ok(input.to_ascii_lowercase())
}

pub fn result_function_first_ok_no_match_components(
    input: Result<String, String>,
) -> Result<String, String> {
    let input = match input {
        Ok(input) => input,
        err => return err,
    };
    Ok(input.to_ascii_lowercase())
}

pub fn result_function_first_ok_no_match_components_after_fix(
    input: Result<String, String>,
) -> Result<String, String> {
    let input = input?;
    Ok(input.to_ascii_lowercase())
}

pub fn result_function_first_err(input: Result<String, String>) -> Result<String, String> {
    let input = match input {
        Err(err) => return Err(err),
        Ok(input) => input,
    };
    Ok(input.to_ascii_lowercase())
}

pub fn result_function_first_err_after_fix(
    input: Result<String, String>,
) -> Result<String, String> {
    let input = input?;
    Ok(input.to_ascii_lowercase())
}
