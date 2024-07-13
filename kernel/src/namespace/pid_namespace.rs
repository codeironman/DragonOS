use alloc::sync::Arc;

use crate::{libs::rwlock::RwLock, process::Pid};

struct PidNamespace {
    /// 已经分配的进程数
    pid_allocated : u32,
    /// 当前的pid_namespace所在的层数
    level : u32,
    /// 父命名空间
    parent : Arc<PidNamespace>,
    /// 资源计数器
    ucount : Arc<Ucount>,
    /// 关联的用户namespace
    user_ns : Arc<UserNamespace>,
    /// 回收孤儿进程的init进程
    child_reaper : Arc<RwLock<Pid>>,
    /// namespace共有部分
    common_ns : NsCommon,
}

impl PidNamespace {
    pub fn new(parent : &PidNamespace,user_ns : Arc<UserNamespace>){
        let level = parent.level + 1;
        
    }

}






