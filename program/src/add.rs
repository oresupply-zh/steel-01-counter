use counter_api::prelude::*;
use steel::*;
use solana_program::msg;

/// 处理添加指令的函数
/// 用于增加或减少计数器的值
///
/// # 参数
/// * `accounts` - 指令需要的账户列表
/// * `data` - 指令的输入数据
///
/// # 返回值
/// * `ProgramResult` - 操作结果，成功返回 Ok(())，失败返回错误
pub fn process_add(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // 解析指令参数
    let args = Add::try_from_bytes(data)?;
    // 将 8 字节数组转换为 u64 类型
	let amount = u64::from_le_bytes(args.amount);

    // 获取并验证所需账户
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    // 验证交易签名者
    signer_info.is_signer()?;
    // 加载计数器账户并进行验证
	let counter = counter_info
		.as_account_mut::<Counter>(&counter_api::ID)?
        // 确保计数器值小于 100（安全限制）
		.assert_mut(|c| c.value < 100)?;

    // 更新计数器状态
	counter.value += amount;
    msg!("Counter value updated to {}", counter.value);

    Ok(())
}
