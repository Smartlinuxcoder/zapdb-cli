use zapdb::{Column, DataType, Database, Value};
use std::collections::HashMap;
use std::io::{self, Write};
use colored::*;
use prettytable::{Table, Cell, Row};

#[tokio::main]
async fn main() {
    let mut db = Database::new();

    println!("{}", "Welcome to the goofy ZapDB Shell!".bright_blue().bold());

    loop {
        print!("{}", "ZapDB shell> ".bright_green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                println!("{}", "Goodbye!".bright_blue().bold());
                break;
            }
            "echo" => {
                println!("{}", args.join(" ").cyan());
            }
            "create_table" => {
                if args.len() < 2 {
                    println!("{}", "Usage: create_table <table_name> <column1:type> ...".red());
                    continue;
                }
                let table_name = args[0];
                let mut columns = Vec::new();
                
                for col_def in &args[1..] {
                    let parts: Vec<&str> = col_def.split(":").collect();
                    if parts.len() != 2 {
                        println!("{}", format!("Invalid column definition: {}", col_def).red());
                        continue;
                    }
                    let data_type = match parts[1] {
                        "string" => DataType::String,
                        "int" => DataType::Integer,
                        _ => {
                            println!("{}", format!("Unsupported data type: {}", parts[1]).red());
                            continue;
                        }
                    };
                    columns.push(Column {
                        name: parts[0].to_string(),
                        data_type,
                    });
                }
                
                match db.create_table(table_name.to_string(), columns) {
                    Ok(_) => println!("{}", format!("Table {} created successfully", table_name).green()),
                    Err(e) => println!("{}", format!("Error creating table: {:?}", e).red()),
                }
            }
            "insert" => {
                if args.len() < 3 {
                    println!("{}", "Usage: insert <table> <column1=value1> ...".red());
                    continue;
                }
                let table = args[0];
                let mut record = HashMap::new();
                
                for pair in &args[1..] {
                    let parts: Vec<&str> = pair.split("=").collect();
                    if parts.len() != 2 {
                        println!("{}", format!("Invalid key-value pair: {}", pair).red());
                        continue;
                    }
                    let value = if parts[1].parse::<i64>().is_ok() {
                        Value::Integer(parts[1].parse().unwrap())
                    } else {
                        Value::String(parts[1].to_string())
                    };
                    record.insert(parts[0].to_string(), value);
                }
                
                match db.insert(table, record) {
                    Ok(_) => println!("{}", "Record inserted successfully".green()),
                    Err(e) => println!("{}", format!("Error inserting record: {:?}", e).red()),
                }
            }
            "select" => {
                if args.is_empty() {
                    println!("{}", "Usage: select <table>".red());
                    continue;
                }
                let table = args[0];
                match db.select(table, None) {
                    Ok(records) => {
                        if records.is_empty() {
                            println!("{}", "No records found".yellow());
                            continue;
                        }
                        //OUCH MY EYES
                        //TODO: make this a function or something that doesn't hurt to look at
                        let mut table = Table::new();
                        
                        let headers: Vec<_> = records[0].keys().collect();
                        
                        let header_row = Row::new(
                            headers.iter()
                                .map(|h| Cell::new(h).style_spec("bFB"))  // Blue headers
                                .collect()
                        );
                        table.add_row(header_row);

                        for record in records {
                            let row = Row::new(
                                headers.iter()
                                    .map(|&key| {
                                        let (value, style) = match record.get(key) {
                                            Some(&Value::Integer(i)) => (i.to_string(), "Fg"),     // Green for integers
                                            Some(&Value::String(ref s)) => (s.clone(), "Fc"),      // Cyan for strings
                                            Some(&Value::Float(f)) => (f.to_string(), "Fm"),       // Magenta for floats
                                            Some(&Value::Boolean(b)) => (b.to_string(), "Fy"),     // Yellow for booleans
                                            Some(&Value::Null) | None => ("NULL".to_string(), "Fr")  // Red for null values
                                        };
                                        Cell::new(&value).style_spec(style)
                                    })
                                    .collect()
                            );
                            table.add_row(row);
                        }

                        // High stakes gambling
                        println!("\nValue Types:");
                        println!("{}", "INTEGER".green());
                        println!("{}", "STRING".cyan());
                        println!("{}", "FLOAT".magenta());
                        println!("{}", "BOOLEAN".yellow());
                        println!("{}", "NULL".red());
                        println!();  

                        table.printstd();
                    }
                    Err(e) => println!("{}", format!("Error selecting records: {:?}", e).red()),
                }
            }
            "save" => {
                if args.is_empty() {
                    println!("{}", "Usage: save <filename>".red());
                    continue;
                }
                match db.save(args[0]).await {
                    Ok(_) => println!("{}", "Database saved successfully".green()),
                    Err(e) => println!("{}", format!("Error saving database: {:?}", e).red()),
                }
            }
            "load" => {
                if args.is_empty() {
                    println!("{}", "Usage: load <filename>".red());
                    continue;
                }
                match db.load(args[0]).await {
                    Ok(_) => println!("{}", "Database loaded successfully".green()),
                    Err(e) => println!("{}", format!("Error loading database: {:?}", e).red()),
                }
            }
            "" => continue,
            _ => println!("{}", format!("Unknown command: {}", command).red()),
        }
    }
}