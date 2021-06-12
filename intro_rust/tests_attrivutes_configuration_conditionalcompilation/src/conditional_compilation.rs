#[cfg(target_os = "linux")]
pub fn are_you_on_linux() {
    println!("Greeting for linux!");
}

#[cfg(not(target_os = "linux"))]
pub fn are_you_on_linux() {
    println!("Greeting for other OS!");
}

pub fn do_something() {
    if cfg!(target_os = "linux") {
        println!("I am in Linux");
    } else {
        println!("I am in other OS");
    }
}
