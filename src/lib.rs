extern crate time;

use std::io::*;
use std::fs::File;
use std::fmt;

/// Custom result with ChlogError
type ChlogResult = std::result::Result<(), ChlogError>;

/// Custom error type with errors specific to changelog edition
#[derive(Debug)]
pub enum ChlogError {
    /// Entry is already present in the file
    AlreadyPresent,

    /// Changelog file was not found
    FileNotFound,

    /// Couldn't create a file
    FileWriteFailed(std::io::Error),

    /// Couldn't read a file
    FileReadFailed(std::io::Error),

    /// Couldn't save a file
    FileCreateFailed(std::io::Error),
}

impl fmt::Display for ChlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChlogError::AlreadyPresent =>
                write!(f, "Error: Entry is already present!"),
            ChlogError::FileNotFound =>
                write!(f, "Error: CHANGELOG file not found!"),
            ChlogError::FileWriteFailed(ref e) =>
                e.fmt(f),
            ChlogError::FileReadFailed(ref e) =>
                e.fmt(f),
            ChlogError::FileCreateFailed(ref e) =>
                e.fmt(f),
        }
    }
}

type ReadmeResult = std::result::Result<(), ReadmeError>;

#[derive(Debug)]
pub enum ReadmeError {
    FileNotFound,
    FileWriteFailed(std::io::Error),
    FileReadFailed(std::io::Error),
}

impl fmt::Display for ReadmeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadmeError::FileNotFound =>
                write!(f, "Error: CHANGELOG file not found!"),
            ReadmeError::FileWriteFailed(ref e) =>
                e.fmt(f),
            ReadmeError::FileReadFailed(ref e) =>
                e.fmt(f),
        }
    }
}

/// # add_exercise
/// Adds an exercise to the changelog
/// ## Arguments:
/// language: Programming language of the exercise
/// name: Name of the exercise
/// source: Source of the exercise (e.g. CodeWars)
/// file_path: path to the changelog file
/// ## Return
/// Returns ChlogResult (Result<(), ChlogError>)
/// * Ok(()) if no errors occurred
/// * Err(ChlogError) if error occurred
pub fn add_exercise(language: &str, name: &str,
                    source: &str, file_path: &str) -> ChlogResult {
    let mut buff = String::new();

    if let Ok(mut chlog) = File::open(file_path) {
        if let Err(e) = chlog.read_to_string(&mut buff) {
            return Err(ChlogError::FileReadFailed(e));
        }

        let tm = time::now();
        let line = format!("#### {}.{:02}.{}\n",
                           tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);
        let exercise = format!("* [{}] {} ({})", language, name, source);

        if let Some(idx) = buff.find(line.as_str()) {
            if block_contains(&mut buff, &line, &exercise) {
                return Err(ChlogError::AlreadyPresent);
            }

            buff.insert_str(idx+line.len()+1,
                            format!("{}\n", exercise).as_str());
        } else if let Some(idx) = buff.find("####") {
            buff.insert_str(idx,
                            format!("{}\n{}\n\n", line, exercise).as_str());
        } else if let Some(idx) = buff.find("\n") {
            buff.insert_str(idx, format!("\n\n{}\n{}", line, exercise).as_str());
        } else {
            buff.insert_str(0, format!("{}\n{}", line, exercise).as_str());
        }
    } else {
        return Err(ChlogError::FileNotFound);
    }

    match File::create(file_path) {
        Ok(mut file) => {
            match file.write(buff.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ChlogError::FileWriteFailed(e)),
            }
        },
        Err(e) => Err(ChlogError::FileCreateFailed(e)),
    }
}

/// # add_commit
/// Adds a development commit to the changelog
/// ## Arguments:
/// category: Category of the commit
/// description: Description of the exercise
/// file_path: path to the changelog file
/// ## Return
/// Returns ChlogResult (Result<(), ChlogError>)
/// * Ok(()) if no errors occurred
/// * Err(ChlogError) if error occurred
pub fn add_commit(category: &str, description: &str,
                  file_path: &str) -> ChlogResult {
    let mut buff = String::new();

    if let Ok(mut chlog) = File::open(file_path) {
        if let Err(e) = chlog.read_to_string(&mut buff) {
            return Err(ChlogError::FileReadFailed(e));
        }

        let tm = time::now();
        let header = format!("#### {}.{:02}.{}\n",
                             tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);
        let entry = format!("* [{}] {}", category, description);

        if let Some(idx) = buff.find(header.as_str()) {
            if block_contains(&mut buff, &header, &entry) {
                return Err(ChlogError::AlreadyPresent);
            }

            buff.insert_str(idx+header.len()+1, format!("{}\n", entry).as_str());
        } else if let Some(idx) = buff.find("####") {
            buff.insert_str(idx, format!("{}\n{}\n\n", header, entry).as_str());
        } else if let Some(idx) = buff.find("\n") {
            buff.insert_str(idx, format!("\n\n{}\n{}", header, entry).as_str());
        } else {
            buff.insert_str(0, format!("{}\n{}", header, entry).as_str());
        }
    } else {
        return Err(ChlogError::FileNotFound);
    }

    match File::create(file_path) {
        Ok(mut file) => {
            match file.write(buff.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ChlogError::FileWriteFailed(e)),
            }
        },
        Err(e) => Err(ChlogError::FileCreateFailed(e)),
    }
}

/// # add_learning
/// Adds a learning commit to the changelog
/// ## Arguments:
/// language: Programming language of the commit
/// description: Description of the material
/// source: Source of the material
/// file_path: path to the changelog file
/// ## Return
/// Returns ChlogResult (Result<(), ChlogError>)
/// * Ok(()) if no errors occurred
/// * Err(ChlogError) if error occurred
pub fn add_learning(language: &str, description: &str,
                    source: &str, file_path: &str) -> ChlogResult {
    let mut buff = String::new();

    if let Ok(mut chlog) = File::open(file_path) {
        if let Err(e) = chlog.read_to_string(&mut buff) {
            return Err(ChlogError::FileReadFailed(e));
        }

        let tm = time::now();
        let header = format!("#### {}.{:02}.{}\n",
                             tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);
        let entry = format!("* [{}] {} ({})",
                            language, description, source);

        if let Some(idx) = buff.find(header.as_str()) {
            if block_contains(&mut buff, &header, &entry) {
                return Err(ChlogError::AlreadyPresent);
            }

            buff.insert_str(idx+header.len()+1, format!("{}\n", entry).as_str());
        } else if let Some(idx) = buff.find("####") {
            buff.insert_str(idx, format!("{}\n{}\n\n", header, entry).as_str());
        } else if let Some(idx) = buff.find("\n") {
            buff.insert_str(idx, format!("\n\n{}\n{}", header, entry).as_str());
        } else {
            buff.insert_str(0, format!("{}\n{}", header, entry).as_str());
        }
        return Err(ChlogError::FileNotFound);
    }

    match File::create(file_path) {
        Ok(mut file) => {
            match file.write(buff.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ChlogError::FileWriteFailed(e)),
            }
        },
        Err(e) => Err(ChlogError::FileCreateFailed(e)),
    }
}

/// # add_other
/// Adds an other commit to the changelog
/// ## Arguments:
/// category: Category of the commit
/// description: Description of the exercise
/// file_path: path to the changelog file
/// ## Return
/// Returns ChlogResult (Result<(), ChlogError>)
/// * Ok(()) if no errors occurred
/// * Err(ChlogError) if error occurred
pub fn add_other(category: &str, description: &str,
                  file_path: &str) -> ChlogResult {
    let mut buff = String::new();

    if let Ok(mut chlog) = File::open(file_path) {
        if let Err(e) = chlog.read_to_string(&mut buff) {
            return Err(ChlogError::FileReadFailed(e));
        }

        let tm = time::now();
        let header = format!("#### {}.{:02}.{}\n",
                             tm.tm_mday, tm.tm_mon+1, tm.tm_year+1900);
        let entry = format!("* [{}] {}", category, description);

        if let Some(idx) = buff.find(header.as_str()) {
            if block_contains(&mut buff, &header, &entry) {
                return Err(ChlogError::AlreadyPresent);
            }

            buff.insert_str(idx+header.len()+1, format!("{}\n", entry).as_str());
        } else if let Some(idx) = buff.find("####") {
            buff.insert_str(idx, format!("{}\n{}\n\n", header, entry).as_str());
        } else if let Some(idx) = buff.find("\n") {
            buff.insert_str(idx, format!("\n\n{}\n{}", header, entry).as_str());
        } else {
            buff.insert_str(0, format!("{}\n{}", header, entry).as_str());
        }
    } else {
        return Err(ChlogError::FileNotFound);
    }

    match File::create(file_path) {
        Ok(mut file) => {
            match file.write(buff.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ChlogError::FileWriteFailed(e)),
            }
        },
        Err(e) => Err(ChlogError::FileCreateFailed(e)),
    }
}

pub fn update_readme_exercise_count(language: &str,
                                    file_path: &str)-> ReadmeResult {
    let mut buff = String::new();

    if let Ok(mut readme) = File::open(file_path) {
        if let Err(e) = readme.read_to_string(&mut buff) {
            return Err(ReadmeError::FileReadFailed(e));
        }

    } else {
        return Err(ReadmeError::FileNotFound);
    }

    Ok(())
}

/// # block_contains
/// Checks if a block with a given header contains the specific entry
/// ## Arguments
/// buff: &str - Buffer with contents to be searched
/// block_header: &str - Header of the block to be checked
/// entry: &str - Entry to be checked for
/// ## Return
/// Returns bool
/// * true if block was found and contains the given entry
/// * false if block wasn't found or if it doesn't contain the entry
fn block_contains(buff: &str, block_header: &str, entry: &str) -> bool {
    buff.lines().skip_while(|&l| l != &block_header[..block_header.len()-1])
        .skip(2).take_while(|&l| !l.starts_with("####")).any(|l| l == entry)
}
