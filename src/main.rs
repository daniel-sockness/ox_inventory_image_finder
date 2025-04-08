use rlua::{Lua, Value};
use std::fs;
use std::io;
use std::io::prelude::*;

use std::process;
fn image_exists(img: &str) -> bool {
    let image_path = format!("web/images/{}", img);
    if std::path::Path::new(&image_path).exists() {
        return true;
    } else {
        return false;
    }
}

fn parse_lua_from_file(file_path: &str) -> rlua::Result<()> {
    // Read the Lua code from the file
    let lua_code = fs::read_to_string(file_path).expect("Failed to read Lua file");
    let images_dir = "web/images";

    // Create a new Lua instance
    let lua = Lua::new();

    // Load and evaluate the Lua code
    let globals = lua.globals();
    globals.set(
        "vec3",
        lua.create_function(|_, _: ()| {
            // Prevent calling the vec3 function
            Ok(())
        })?,
    )?;

    // Load the Lua table from the file
    let table: Value = lua.load(&lua_code).eval()?;
    let mut count = 0;
    // Check if the Lua result is a table
    if let Value::Table(tbl) = table {
        // Iterate through each key in the table
        for pair in tbl.pairs::<String, Value>() {
            let (key, value) = pair?;

            // Check if the value is a table (client section)
            if let Value::Table(client) = value {
                // Try to get the `client` field
                if let Some(Value::Table(client)) = client.get::<_, Option<Value>>("client")? {
                    // Try to get the `image` field from the client table
                    if let Some(Value::String(image)) = client.get("image")? {
                        // If an image is present, print the image string
                        if image_exists(image.to_str()?) == false {
                            println!("Image does not exist exists: {}", image.to_str()?);
                            count += 1;
                        }
                        continue; // Skip printing the name after image is printed
                    }
                }
            }
            // If no image found, print the name
            //println!("{}", key);
            let name = key.to_string() + ".png";
            if image_exists(&name) == false {
                println!("Image does not exist: {}", name);
                count += 1;
            }
        }
    }
    println!("Found {} missing images", count);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
    process::exit(0);
    Ok(())
}

fn main() {
    let items_path = "data/items.lua";
    let images_path = "web/images";
    // Check if the file exists

    println!("-----------------------------------------------------");
    println!("|         ox_inventory missing image finder         |");
    println!("|                 Made by: Daniel S.                |");
    println!("-----------------------------------------------------");
    if !std::path::Path::new(items_path).exists() && !std::path::Path::new(images_path).exists() {
        println!("This program must be run in the root of the ox_inventory folder");
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
        process::exit(0);
    }
    if let Err(e) = parse_lua_from_file(items_path) {
        eprintln!("Error parsing Lua: {}", e);
    }
}
