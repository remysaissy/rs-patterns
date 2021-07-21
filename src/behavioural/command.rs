
// Database migrations.

// Trait for implementing commands.
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;    
}

// Example commands.
pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str { 
        "create table"
    }
    fn rollback(&self) -> &str { 
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {    
    fn execute(&self) -> &str { 
        "alter table <table> add"
    }
    fn rollback(&self) -> &str { 
        "alter table <table> drop"
    }
}

// Command pattern.
pub struct Schema {
    commands: Vec<Box<dyn Migration>>,
}
impl Schema {
    fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands
                .iter()
                .map(|cmd| cmd.execute())
                .collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands
                .iter()
                .rev()
                .map(|cmd| cmd.rollback())
                .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Schema, CreateTable, AddField};
    
    #[test]
    fn test_execute() {
        let mut schema = Schema::new();

        let cmd = Box::new(CreateTable);
        schema.add_migration(cmd);
        let cmd = Box::new(AddField);
        schema.add_migration(cmd);

        assert_eq!(vec!["create table", "alter table <table> add"], schema.execute());
    }

    #[test]
    fn test_rollback() {
        let mut schema = Schema::new();

        let cmd = Box::new(CreateTable);
        schema.add_migration(cmd);
        let cmd = Box::new(AddField);
        schema.add_migration(cmd);

        assert_eq!(vec!["alter table <table> drop", "drop table"], schema.rollback());
    }
    

}