use xkb_data::{models::KeyboardModels, AccessList};

fn main() {
    println!("Keyboard models");
    let models: KeyboardModels = match xkb_data::section::all_sections() {
        Ok(models) => models,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    let mut total_count = 0;
    for model in models.get_list() {
        println!("  {}: {}", model.name(), model.description());
        total_count += 1;
    }
    println!("All models: {}", total_count);

    println!("Extra-provided models:");
    let models: KeyboardModels = xkb_data::section::fetch_extra_section().unwrap();
    let mut extra_count = 0;
    for model in models.get_list() {
        println!("  {}: {}", model.name(), model.description());
        extra_count += 1;
    }
    println!("Total extra-provided models: {}", extra_count);

    println!("user-provided models:");
    let models: KeyboardModels = xkb_data::section::fetch_user_section().unwrap();
    let mut user_count = 0;
    for model in models.get_list() {
        println!("  {}: {}", model.name(), model.description());
        user_count += 1;
    }
    println!("Total user-provided models: {}", user_count);
}
