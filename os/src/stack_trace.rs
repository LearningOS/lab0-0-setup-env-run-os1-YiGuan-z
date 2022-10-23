use core::arch::asm;
use core::ptr;

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    //使用汇编把fp寄存器的内容剪切到 fp变量里
    asm!("mv {}, fp", out(reg) fp);

    println!("=== Begin stack trace==");
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        //将{}里面的内容解析为16位
        println!("0x{:016x},fp=0x{:016x}", saved_ra, saved_fp);
        fp = saved_fp as *const usize;
    }
    println!("==END stack trace ==");
}