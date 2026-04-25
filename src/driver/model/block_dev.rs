//! 块设备驱动能力模型
//!
//! 块设备驱动需要实现以下函数指针表,注册给OS后,
//! OS通过此表调用驱动的读写能力。
//!
//! 同样使用`#[repr(C)]`+函数指针,保证跨Rust版本二进制兼容。

use core::ffi::c_void;

/// 块设备信息
#[repr(C)]
pub struct BlockDevInfo {
    /// 逻辑块大小(字节)
    pub block_size: u32,
    /// 总块数
    pub block_count: u64,
    /// 设备最大传输大小(字节),0表示无限制
    pub max_transfer_size: u32,
}

/// 块设备驱动对外暴露的能力表
///
/// 驱动负责填充此结构体并注册给OS,`ctx`为驱动私有上下文,
/// 每个函数调用时OS会将`ctx`原样传回。
#[repr(C)]
pub struct BlockDevOps {
    /// 驱动私有上下文
    pub ctx: *mut c_void,

    /// 读取块数据
    /// `block_offset`: 起始块号
    /// `block_count`: 读取块数
    /// `buf`: 目标缓冲区,由调用者分配,大小至少为`block_count * block_size`
    /// 返回0表示成功,非0为错误码
    pub read_blocks: unsafe extern "C" fn(
        ctx: *mut c_void,
        block_offset: u64,
        block_count: u32,
        buf: *mut u8,
    ) -> i32,

    /// 写入块数据
    /// 参数同`read_blocks`, `buf`为源缓冲区
    /// 返回0表示成功,非0为错误码
    pub write_blocks: unsafe extern "C" fn(
        ctx: *mut c_void,
        block_offset: u64,
        block_count: u32,
        buf: *const u8,
    ) -> i32,

    /// 获取设备信息,驱动将信息填入`info`
    /// 返回0表示成功
    pub get_info: unsafe extern "C" fn(ctx: *mut c_void, info: *mut BlockDevInfo) -> i32,
}
