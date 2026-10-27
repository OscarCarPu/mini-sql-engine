use mini_sql_engine::{Database, execute};
use rustyline::DefaultEditor;

fn main() {
    let mut db = Database::default();
    let Ok(mut rl) = DefaultEditor::new() else {
        println!("Error creating the rustyline");
        return;
    };
    while let Ok(line) = rl.readline("minisql> ") {
        let line = line.trim();
        if line == "q" {
            break;
        }
        if line.is_empty() {
            continue;
        }
        match execute(line, &mut db) {
            Ok(output) => println!("{}", output),
            Err(e) => println!("error: {}", e),
        }
    }
}
