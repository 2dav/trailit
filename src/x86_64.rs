use std::{arch, mem, ptr};

use crate::stack::Stack;

pub struct Context {
    ip: *mut usize,
    sp: *mut usize,
}

impl Context {
    #[cfg(any(test, feature = "tests_api"))]
    pub fn get_ip(&self) -> *mut usize {
        self.ip
    }

    #[cfg(any(test, feature = "tests_api"))]
    pub fn get_sp(&self) -> *mut usize {
        self.sp
    }
}

fn trampoline<F: FnOnce(usize, Context) -> Context>(f: F, arg: usize, ctx: Context) -> Context {
    // F is ZST and is inlined here, so the arguments shifted one left - `rdi`:arg, `rsi,rdx`:ctx
    f(arg, ctx)
}

#[naked]
unsafe extern "C" fn trampoline_thunk<F: FnOnce(usize, Context) -> Context>() -> ! {
    arch::asm!(
        // - - - - - - - - - - - - - - - - - - Call into trampolined function
        // `rdi` - argument
        // `rsi` - callers return address
        // `rdx` - callers stack pointer
        "call   {0}",
        "xchg   rsp, rdx",
        "jmp    rax",
        sym trampoline::<F>,
        options(noreturn),
    )
}

#[inline(always)]
pub unsafe fn switch(ctx: &mut Context, mut arg: usize) -> usize {
    arch::asm!(
        // Save `rbx` and `rbp` because we can't use them in a clobber since both are reserved by llvm
        "push   rbx",
        "push   rbp",

        "xchg   rsp, rdx",
        "lea    rsi, [rip + 0f]",
        "jmp    rax",
        "0:",

        // Restore `rbx` `rbp` which we've saved earlier because we can't use them as a clobber.
        "pop    rbp",
        "pop    rbx",

        in("rax") ctx.ip,
        lateout("rsi") ctx.ip,
        inlateout("rdx") ctx.sp,
        inlateout("rdi") arg,
        // clobbers
        lateout("rax") _, lateout("rcx") _,  
        lateout("r12") _, lateout("r13") _,lateout("r14") _,lateout("r15") _,
        clobber_abi("sysv64"),
        //options(may_unwind),
    /* Options:
        rustc emits the following clobbers,
        - by *not* specifying `options(preserves_flags)`:
            (x86) ~{dirflag},~{flags},~{fpsr}
            (ARM/AArch64) ~{cc}
        - by *not* specifying `options(nomem)`:
            ~{memory}
        - by *not* specifying `nostack`:
            alignstack
    */);
    arg
}

pub unsafe fn it<F: FnOnce(usize, Context) -> Context>(f: F, stack: &mut Stack) -> Context {
    let base = Context { ip: ptr::null_mut(), sp: stack.base().cast() };
    form_stack(f, base)
}

unsafe fn form_stack<F: FnOnce(usize, Context) -> Context>(_: F, mut ctx: Context) -> Context {
    assert_eq!(mem::size_of::<F>(), 0, "stateful closures not supported yet");
    ctx.ip = trampoline_thunk::<F> as *mut usize;
    ctx
}
