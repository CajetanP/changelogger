# Changelog

### All notable changes to the program should be listed in this file

---

#### 31.08.2017

* [add_exercise] Fixed wrong day placement if file contains additional lines

#### 30.08.2017

* [block_contains] Refactored block\_contains to remove unnecessary variable
* [Doc comments] Adjusted doc comments for all functions in lib.rs
* [Doc comments] Added doc comments for block_contains

#### 29.08.2017

* Cleaned up unnecessary comments
* Improved add\_learning function to use block\_contains
* Improved add\_commit function to use block\_contains

#### 28.08.2017

* Improved add\_exercise function to make use of block\_contains
* Added block_contains function checking if entry is already present in a block
* Implemented add_learning function

#### 27.08.2017

* Fixed a mistake
* Separated app and matches in the main function

#### 26.08.2017

* Started add_learning function
* Fixed crashing when CHANGELOG file has few lines
* Finished add_commit function

#### 25.08.2017

* Started implementing add_commit function
* Documentation comments
* Custom error handling

#### 24.08.2017

* Optional command line argument with a file path
* Creating a CHANGELOG file if none present
* Returning with a message if there's no CHANGELOG file found
* Returning with a message if exercise is already present
* Adding a new day if it doesn't already exist

#### 23.08.2017

* Writing desired string to the file and saving it
* Finding an appropriate line in the file and inserting something
* Opening a file

#### 22.08.2017

* Created function add_exercise to which cli arguments are passed
* Added clap crate, set up basic command line argument parsing
* Set up a cargo project
* Started a project, added README, LICENSE, CHANGELOG
