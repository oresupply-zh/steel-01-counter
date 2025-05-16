# 计数器程序

**Counter** 是一个使用 Steel 框架构建的 Solana 程序，用于演示基本的计数器功能。该项目作为教程示例，展示如何使用 Steel 框架构建和组织 Solana 程序。
        
## API 接口
- [`Consts`](api/src/consts.rs) – 程序常量定义
- [`Error`](api/src/error.rs) – 自定义程序错误
- [`Event`](api/src/event.rs) – 程序事件定义
- [`Instruction`](api/src/instruction.rs) – 指令声明

## 核心组件实现

### 指令定义
```rust
// api/src/instruction.rs
pub enum CounterInstruction {
    Initialize = 0,  // 初始化计数器
    Add = 1         // 增加计数器值
}

// Add 指令的参数结构
pub struct Add {
    pub amount: [u8; 8]  // 要增加的数量，以 8 字节数组形式存储
}
```

### 计数器状态
```rust
// api/src/state/counter.rs
pub struct Counter {
    pub value: u64  // 计数器当前值
}
```

### 关键操作

#### 初始化操作
```rust
// program/src/initialize.rs
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // 1. 验证所需账户：签名者、计数器账户、系统程序
    // 2. 创建新的计数器账户
    // 3. 初始化计数器值为 0
}
```

#### 增加操作
```rust
// program/src/add.rs
pub fn process_add(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // 1. 解析输入参数
    // 2. 验证签名者和计数器账户
    // 3. 确保计数器值小于 100
    // 4. 更新计数器值
}
```

### 安全特性
- 所有操作都需要签名者验证
- 计数器值有上限限制（小于 100）
- 使用 Steel 框架的安全特性进行账户验证

## 指令说明
- [`Add`](program/src/add.rs) – 增加计数器的值
- [`Initialize`](program/src/initialize.rs) – 初始化一个新的计数器账户

## 状态管理
- [`Counter`](api/src/state/counter.rs) – 计数器状态结构及管理

## 开始使用

编译程序：
```sh
steel build
```

运行单元测试和集成测试：
```sh
steel test
```
