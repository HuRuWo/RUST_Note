#### 第一步导入main.rs

```rust
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
```

直接run报错 因为所有的mod都是不存在的，需要接下来自己编写。
mod就是单独的一块引入代码，可以用 `mod`关键字也可以将一个单独的rs文件作为mod


#### 新建rs文件解决mod依赖问题

新建五个rs文件

```rust
args.rs
chunk.rs
chunk_type.rs
commands.rs
png.rs
```

#### 实现 chunk_type.rs 逻辑

chunk_type.rs 区块类型，实现png图片
