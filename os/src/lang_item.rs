// os/src/lang.rs
use core::panic::{PanicInfo};
use crate::sbi::shutdown;
use crate::stack_trace::print_stack_trace;


// 推测 '->!' 为永远无法到达,与ts的naver类型相似
// 将错误处理函数替换为我们自己的实现
#[panic_handler]
#[allow(unused_unsafe)]
unsafe fn panic(info: &PanicInfo) -> ! {
    match { info.location() } {
        None => {
            //如果没有，就打印当前日志
            println!("Panicked:{}", info.message().unwrap())
        }
        Some(location) => {
            //打印本地错误信息
            println!("Panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap())
        }
    }
    //打印异常堆栈
    unsafe {
        print_stack_trace();
    }
    //关闭程序
    shutdown()
}
