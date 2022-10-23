// os/src/main.rs
//移除std标准库
#![no_std]
//移除main函数
#![no_main]
//移除错误堆栈
#![feature(panic_info_message)]


use log::{debug, error, info, trace, warn};

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod logging;
mod stack_trace;
//导入内存分布图
core::arch::global_asm!(include_str!("entry.asm"));
//将内存全部设为0
//开始将程序段内存清零
fn clear_bss() {
    //导入外部C代码
    extern "C" {
        fn sbss();
        fn ebss();
    }
    //将两端内存看成是数字，然后全部清零
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

//操作系统的启动入口
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        //执行在链接文件内定义的内存分配方法
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    //内存分配完成先清零
    clear_bss();
    //日志系统初始化
    logging::init();
    //打印字符串
    println!("system start!");
    let retcode = run();
    if retcode == 1 { panic!("System Error!\n") }

    //根据级别来决定是否打印其他东西
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    //使用的是我们自己的panic函数，该函数将错误打印完毕就关机
    panic!("Shutdown machine!\n");
}


const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}


//操作系统启动成功后函数执行入口
pub fn run() -> usize {
    let mut count: usize = 0;
    loop {
        print!("{}\t", count);
        count += 1;
        if count%10==0 {
            print!("\n");
        }
        if count >= 100 {
            break;
        }
    }
    return 0;
}


