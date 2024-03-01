use std::env;

pub fn list_requests(){
    // Get current directory
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let requests_path = cwd.join("requests");

    println!("Listing requests in: {}", requests_path.display());

}
