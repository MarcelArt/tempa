pub fn snake_case(text: String) -> String {
    text.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace(".", "_")
}

pub fn pascal_to_snake_case(text: String) -> String {
    let mut result = String::new();
    let mut prev_char = ' ';
    for c in text.chars() {
        if c.is_uppercase() && !prev_char.is_whitespace() {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
        prev_char = c;
    }
    result
}

#[test]
fn test_pascal_to_snake() {
    assert_eq!(pascal_to_snake_case("GameManager".to_string()), "game_manager");
}

#[test]
fn test_snake_case() {
    assert_eq!(snake_case(String::from("Half-Life")), "half_life");
    assert_eq!(snake_case(String::from("Dark Souls")), "dark_souls");
    assert_eq!(snake_case(String::from("godot.rust")), "godot_rust");
}