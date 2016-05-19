pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(str_name) => { format!("Hello, {}!", str_name) }
        None => "Hello, World!".to_string()
    }
}
