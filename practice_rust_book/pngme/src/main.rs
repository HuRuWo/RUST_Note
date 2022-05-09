mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>; //定义公共变量
pub type Result<T> = std::result::Result<T, Error>; //定义范型


fn main() -> Result<()> {
    todo!()
}