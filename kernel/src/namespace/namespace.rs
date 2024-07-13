use core::sync::atomic::{AtomicU32, AtomicUsize};

use alloc::{boxed::Box, sync::Arc};

use crate::libs::rwlock::RwLock;

const UCOUNT_COUNTS: usize = 16;
const UCOUNT_RLIMIT_COUNTS: usize =  16;


lazy_static! {
    static ref  COUNT_MANAGER: COUNTMANAGER = 

}


struct UCount {
    ns : Arc<UserNamspace>,
    uid : u32,
    count : AtomicU32,
    ucount : [AtomicUsize;UCOUNT_COUNTS],
    rlimit : [AtomicUsize;UCOUNT_RLIMIT_COUNTS],
}

struct COUNTMANAGER{
    counts : RwLock<>
}



struct NsCommon {
    /// namespace的操作闭包
    ops : Box<dyn NsOperation>,
    /// 名称空间的标识符
    inum : u32,
}

trait NsOperation {

}

enum NsType{
    PidNamespace,
    UserNamespace,
    UtsNamespace,
    IpcNamespace,
    NetNamespace,
    MntNamespace,
    CgroupNamespace,
    TimeNamespace
}