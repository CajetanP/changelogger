## Readme & Changelog Manager

A basic CLI program for updating changelog files.

Template: "changelogger --add_exercise [language] [title] [source]"
"changelogger --add_learning [language] [material] [source]"
"changelogger --add_commit [category] [description]"

**changelogger --add_exercise Rust "Tank Truck" CodeWars**
**changelogger --add_learning Java "Collections library" "Java in a Nutshell"**
**changelogger --add_commit text-editor "Cleanup and refactoring"**

* Should be able to append to the current day or add it if necessary
* Should change both CHANGELOG.md and README.md
* Maybe some config file with set changelogs and readmes for different things?
