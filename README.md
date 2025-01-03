# ZapDB CLI

A shell for interacting with ZapDB, a simple in-memory database. This `shell` allows you to create tables, insert records, query data, and save the database to disk.

## Installation

2. Clone this repository
3. Run `cargo build --release`
4. The get the ninary in `target/release/zapdb-cli`
//TODO: install via crate
## Usage

Start the CLI by running `zapdb-cli`. You'll be greeted with an interactive shell:


### Available Commands

- `create_table <table_name> <column1:type> ...`  
  Create a new table with specified columns and their types.  
  Types: `string`, `int`  
  Example: `create_table users name:string age:int`

- `insert <table> <column1=value1> ...`  
  Insert a new record into a table.  
  Example: `insert users name=Alice age=25`

- `select <table>`  
  Display all records from a table with color-coded value types:
  - INTEGER (green)
  - STRING (cyan)
  - FLOAT (magenta)
  - BOOLEAN (yellow)
  - NULL (red)

- `save <filename>`  
  Save the current database state to a file.  
  Example: `save johnDatabase.zap`

- `load <filename>`  
  Load a previously saved database.  
  Example: `load dataThatIsSavedToDisk.zap`

- `echo <text>`  
  Print text to the console

- `exit`  
  Exit the shell

## Dependencies

- colored (2.0) - For cooooolors
- prettytable-rs (0.10) - For the goofy table lookin ahh select
- tokio (1.0) - Because async you know?
- zapdb (1.0.4) - The underlying database

## License

This project is distributed under the GNU GPL V3 