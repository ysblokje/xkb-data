fn main() {
    println!("All know sections from xkb");
    let xkb_registry = xkb_data::xkb_registry();
    println!("{xkb_registry:?}");
    let mut count = 0;
    match xkb_registry {
        Ok(definitions) => {
            let options = definitions.options;
            // if let Some(options) = options {
            for group in options.group {
                println!("  {}: {}", group.name(), group.description());
                for value in group.options() {
                    println!(
                        "    {}: {}",
                        value.config_item.name, value.config_item.description
                    );
                }
                count += 1;
            }
            // }
        }
        Err(error) => println!("Didn't find options: {error}"),
    }

    let all_options = xkb_data::all_xkb_options();
    count = 0;
    match all_options {
        Ok(options) => {
            for group in options.option_list.group {
                println!("  {}: {}", group.name(), group.description());
                for value in group.options() {
                    println!(
                        "    {}: {}",
                        value.config_item.name, value.config_item.description
                    );
                }
                count += 1;
            }
        }
        Err(error) => println!("Didn't find options: {error}"),
    }
    println!("Total options including extra sources: {}", count);
}
