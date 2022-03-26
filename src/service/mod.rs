use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, KvError, MemTable, Storage,
};

/// 对Command处理的抽象
pub trait CommandService {
    /// 处理command，返回response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// 从Request中得到Response，目前处理HGET/HGETALL/GSET
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    todo!()
}
