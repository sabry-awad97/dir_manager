fn main() {
    // If the dir_manager::get_args function returns an Ok(config) value, use Result
    // ::and_then to pass the config to dir_manager::run.
    if let Err(e) = dir_manager::get_args().and_then(dir_manager::run) {
        // error print line
        eprintln!("{}", e);

        // Exit the program with a nonzero value to indicate an error.
        std::process::exit(1);
    }
}
