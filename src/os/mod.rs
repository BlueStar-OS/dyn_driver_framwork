//! OS需要向驱动暴露的运行时能力
//!
//! OS是单例,因此这里的函数指针不需要`ctx`参数——
//! OS实现方直接通过全局状态/静态变量获取自身信息即可。

use core::ffi::c_void;

pub struct PhyAddr(pub usize);
pub struct VirAddr(pub usize);
pub struct DmaAddr(pub usize);

/// 中断处理函数, `ctx`为驱动注册时传入的私有上下文
pub type IrqHandler = unsafe extern "C" fn(ctx: *mut c_void);
/// 线程入口函数, `ctx`为创建线程时传入的上下文
pub type ThreadEntry = unsafe extern "C" fn(ctx: *mut c_void);

/// OS运行时的服务表,驱动通过此表调用OS能力
#[repr(C)]
pub struct DriverRuntime {
    // ── 时间 ──
    pub get_current_time_ns: unsafe extern "C" fn() -> u64,
    pub sleep_ms: unsafe extern "C" fn(ms: u64),

    // ── 线程 ──
    pub run_thread: unsafe extern "C" fn(entry: ThreadEntry, thread_ctx: *mut c_void),

    // ── DMA内存 ──
    /// 返回虚拟地址,物理地址写入`phy_out`
    pub alloc_dma_memory:
        unsafe extern "C" fn(size: usize, align: usize, phy_out: *mut usize) -> *mut c_void,
    pub free_dma_memory:
        unsafe extern "C" fn(ptr: *mut c_void, size: usize, align: usize),

    // ── 中断 ──
    pub request_irq: unsafe extern "C" fn(
        irq: u32,
        handler: IrqHandler,
        handler_ctx: *mut c_void,
    ) -> bool,
    pub free_irq: unsafe extern "C" fn(irq: u32),

    // ── 日志 ──
    pub log: unsafe extern "C" fn(msg: *const u8, len: usize),
}
