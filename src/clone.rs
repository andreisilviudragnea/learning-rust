struct Struct {
    field: String,
}

fn another_function(string: String) {
    println!("{string}");
}

pub fn function() {
    let mut v = Struct { field: "".to_string() };
    another_function(v.field.clone());
    v.field = "a".to_string();
}
