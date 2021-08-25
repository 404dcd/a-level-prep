# a-level-prep
To specify which of the tasks to run, give an argument on the command line.

`./a-level-prep X` or `cargo run X`, and for X choose one of:

`hangman`, `gtin1`, `gtin2`, `gtin3`, `server`.

"server" is the extension task. The three GTIN ones are from the controlled assessment material.

### Warning
Compiling this project for the first time will take... several minutes. This is 99.99% due to the extension task, as I had to include a whole web framework.
To mitigate this problem (if you're not interested in running the server), just compile the src/ files individually with rustc.

Make sure to run from the cargo root, or else it can't find the input files.

### Server
The extension task runs a web server with two routes of interest: `/` and `/api/list`.
The former provides a GUI for a student to submit their details, and the latter an API which provides CSV data for all users in the database.

The database is stored in memory, and is not persistent. Just change the path in the `Rocket.toml` file to make it persistent,
but you may have to update the SQL in `init_db`.

### Note
The gtin3 program creates and writes a file called `orders.csv`. It does not write anything to stdout.
