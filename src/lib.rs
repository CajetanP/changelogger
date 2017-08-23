
extern crate time;

use std::io::*;
use std::fs::File;

// TODO: should return result with proper error handling
pub fn add_exercise(language: &str, name: &str, source: &str) {
    println!("adding exercise: [{}] {} ({})", language, name, source);

    if let Ok(mut chlog) = File::open("../test_files/CHANGELOG.md") {
        let mut buff = String::new();

        match chlog.read_to_string(&mut buff) {
            Ok(_) => {
                let tm = time::now();
                println!("{:?}", tm);
                let line = format!("#### {}.{:02}.{}\n",tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);

                if let Some(idx) = buff.find(line.as_str()) {
                    buff.insert_str(idx+line.len()+1, "!test!\n");
                }

                println!("buff: {}", buff);

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
