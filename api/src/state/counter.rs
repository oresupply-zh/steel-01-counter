use steel::*;

use super::CounterAccount;

/// 计数器状态结构体
/// 用于存储计数器的当前值
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    /// 计数器的当前值
    /// 使用无符号 64 位整数存储
    /// 范围: 0 到 2^64-1
    pub value: u64 
}

// 使用 Steel 框架的宏将 Counter 注册为账户类型
account!(CounterAccount, Counter);
