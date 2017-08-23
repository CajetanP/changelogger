
extern crate time;

use std::io::*;
use std::fs::File;

// TODO: should return result with proper error handling
pub fn add_exercise(language: &str, name: &str, source: &str) {

    let mut buff = String::new();
    if let Ok(mut chlog) = File::open("../test_files/CHANGELOG.md") {

        match chlog.read_to_string(&mut buff) {
            Ok(_) => {
                let tm = time::now();
                let line = format!("#### {}.{:02}.{}\n",tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);

                // TODO: if not found, add a new day at the top
                if let Some(idx) = buff.find(line.as_str()) {
                    let exercise = format!("* [{}] {} ({})", language, name, source);
                    buff.insert_str(idx+line.len()+1, format!("{}\n", exercise).as_str());
                }
            },
            Err(e) => panic!("err: {}", e),
        }
    } else {
        // TODO: possibly create it?
        panic!("Changelog not found!");
    }

    if let Ok(mut file) = File::create("../test_files/CHANGELOG.md") {
        if let Err(e) = file.write(buff.as_bytes()) {
            panic!("err: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    // no test
}
