## Readme & Changelog Manager

Not very complicated, cli program. It should be able to update readme and changelog file with completed programming exercises.

* Possible different modes for learning / developing projects
* it should open file -> update entries -> close file
* Perhaps written in Rust ?
* cli management libraries might come in handy

Like "changelogger --add_exercise [language] [title] [source]"
"changelogger --add_learning [language] [material] [source]"
"changelogger --add_commit [project] [description]"

**changelogger --add_exercise Rust "Tank Truck" CodeWars**
**changelogger --add_learning Java "Collections library" "Java in a Nutshell"**
**changelogger --add_commit text-editor refactor**

* Should be able to append to the current day or add it if necessary
* Should change both CHANGELOG.md and README.md
* Maybe some config file with set changelogs and readmes for different things?
