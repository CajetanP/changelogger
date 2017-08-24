extern crate time;

use std::io::*;
use std::fs::File;

// TODO: should return result with proper error handling
pub fn add_exercise(language: &str, name: &str, source: &str, path: Option<&str>) {
    let mut buff = String::new();

    let file_path = match path {
        Some(s) => s,
        None => "CHANGELOG.md",
    };

    if let Ok(mut chlog) = File::open(file_path) {
        if let Ok(_) = chlog.read_to_string(&mut buff) {
            let tm = time::now();
            let line = format!("#### {}.{:02}.{}\n",tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);
            let exercise = format!("* [{}] {} ({})", language, name, source);

            if let Some(idx) = buff.find(line.as_str()) {
                if exercise.as_str() !=
                    &buff[idx+line.len()+1..idx+line.len()+1+exercise.len()] {

                        buff.insert_str(idx+line.len()+1, format!("{}\n", exercise).as_str());
                    } else {
                        println!("Exercise already present!");
                        return;
                    }
            } else {
                if let Some(idx) = buff.find("\n") {
                    buff.insert_str(idx+1, format!("\n{}\n{}\n", line, exercise).as_str());
                }
            }
        }
    } else {
        // TODO: possibly create it?
        println!("Changelog file not found!");
        return;
    }

    if let Ok(mut file) = File::create(file_path) {
        if let Err(e) = file.write(buff.as_bytes()) {
            panic!("err: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    // no test
}
