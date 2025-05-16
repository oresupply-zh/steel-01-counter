use counter_api::prelude::*;
use steel::*;

/// 处理初始化指令的函数
/// 创建并初始化一个新的计数器账户
///
/// # 参数
/// * `accounts` - 指令需要的账户列表
/// * `_data` - 指令的输入数据（本指令不需要额外数据）
///
/// # 返回值
/// * `ProgramResult` - 操作结果，成功返回 Ok(())，失败返回错误
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // 获取并验证所需账户
    let [signer_info, counter_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    // 验证交易签名者
    signer_info.is_signer()?;
    // 验证计数器账户：必须为空、可写入、具有正确的种子
    counter_info.is_empty()?.is_writable()?.has_seeds(
        &[COUNTER],
        &counter_api::ID
    )?;
    // 验证系统程序账户
    system_program.is_program(&system_program::ID)?;

    // 创建计数器程序账户
    create_program_account::<Counter>(
        counter_info,
        system_program,
        signer_info,
        &counter_api::ID,
        &[COUNTER],
    )?;
    // 获取计数器账户并初始化值为 0
    let counter = counter_info.as_account_mut::<Counter>(&counter_api::ID)?;
    counter.value = 0;

    Ok(())
}
