use steel::*;

/// 计数器程序支持的指令枚举
/// 用于定义程序可以执行的所有操作类型
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CounterInstruction {
    /// 初始化指令 - 创建并初始化一个新的计数器账户
    Initialize = 0,
    /// 添加指令 - 修改计数器的值
    Add = 1
}

/// 初始化指令的参数结构
/// 由于初始化不需要额外参数，因此结构体为空
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

/// 添加指令的参数结构
/// 用于指定要增加或减少的数值
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    /// 要改变的数量，使用 8 字节数组存储
    /// 可以表示正数（增加）或负数（减少）
    pub amount: [u8; 8]
}

// 使用 Steel 框架的宏来注册指令
instruction!(CounterInstruction, Initialize);
instruction!(CounterInstruction, Add);
