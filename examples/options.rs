fn main() {
    let options = xkb_data::options::all_xkb_options();
    let mut total_count = 0;
    match options {
        Ok(options) => {
            total_count = options.option_list.group.len();
        }
        Err(error) => println!("Didn't find options: {error}"),
    }
    println!("Total options: {}", total_count);

    let all_options = xkb_data::options::user_xkb_options();
    let mut user_count = 0;
    match all_options {
        Ok(options) => {
            user_count = options.option_list.group.len();
        }
        Err(error) => println!("Didn't find options: {error}"),
    }
    println!("Total user options: {}", user_count);

    let all_options = xkb_data::options::extra_xkb_options();
    let mut extra_count = 0;
    match all_options {
        Ok(options) => {
            extra_count = options.option_list.group.len();
        }
        Err(error) => println!("Didn't find options: {error}"),
    }
    println!("Total options in extra: {}", extra_count);
}
