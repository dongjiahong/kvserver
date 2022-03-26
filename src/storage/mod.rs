mod memory;
use crate::{KvError, Kvpair, Value};

/// 对存储的抽象，我们不关心数据存在哪，但需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个HashTable里获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个HashTable里设置一个key的value，返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看HashTable中是否有
}
