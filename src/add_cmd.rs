use std::fs;

use crate::utils::pascal_to_snake_case;

pub fn execute(node_name: String, base_node: String) -> Result<(), String> {
    let template = format!(
        r#"use godot::{{classes::{{I{base}, {base}}}, prelude::*}};

#[derive(GodotClass)]
#[class(base={base})]
struct {name} {{
    base: Base<{base}>,

    // Change or add your own properties here
    #[export]
    value: i32,
}}

#[godot_api]
impl I{base} for RustExample {{
    fn init(base: Base<{base}>) -> Self {{
        Self {{
            base,
            value: 0,
        }}
    }}

    fn ready(&mut self) {{
        // This is where you would put your initialization code
        godot_print!("{name} is ready!");
    }}

    fn process(&mut self, _delta: f64) {{
        // This is where you would put your game logic
        godot_print!("Hello from Rust!");
    }}

    fn physics_process(&mut self, _delta: f64) {{
        // This is where you would put your physics logic
        godot_print!("Hello from Rust in physics process!");
    }}    
}}     
        "#,
        name = node_name,
        base = base_node,
    );

    let file_name = pascal_to_snake_case(node_name.clone());
    let node_path = format!("src/{}.rs", file_name);
    fs::write(node_path, template).map_err(|e| e.to_string())?;

    Ok(())
}