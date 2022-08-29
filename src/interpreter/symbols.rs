/// All the symbols that can be stored in the symbol table
#[derive(Debug, Clone)]
pub enum Symbol {
    Integer(i32),
    Float(f32),
    String(String),
}