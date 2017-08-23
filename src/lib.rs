
use std::io::*;
use std::fs::File;

// TODO: should return result with proper error handling
pub fn add_exercise(language: &str, name: &str, source: &str) {
    println!("adding exercise: [{}] {} ({})", language, name, source);

    if let Ok(mut chlog) = File::open("../test_files/CHANGELOG.md") {
        let mut buff = String::new();

        match chlog.read_to_string(&mut buff) {
            Ok(_) => {
                println!("buff:\n{}", buff);
            },
            Err(e) => println!("err: {}", e),
        }
    } else {
        panic!("Changelog not found!");
    }

}

#[cfg(test)]
mod tests {
    // no test
}
