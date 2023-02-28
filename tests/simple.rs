use std::collections::HashMap;

use trailit::{it, stack::Stack, switch, Context};

#[test]
fn run_to_completion_empty() {
    unsafe {
        let mut stack = Stack::new(1 << 10);
        let mut ctx = it(|_, ctx| ctx, &mut stack);

        let arg = switch(&mut ctx, 42);
        // given empty 'callee' `rdi` should be untouched we get it back as is
        assert_eq!(arg, 42);
        assert_eq!(ctx.get_sp().cast(), stack.base());
    }
}

#[test]
fn ping_pong_to_completion_empty() {
    fn simple(arg: usize, mut ctx: Context) -> Context {
        let _ = unsafe { switch(&mut ctx, arg + 1) };
        ctx
    }

    unsafe {
        let mut stack = Stack::new(1 << 10);
        let mut ctx = it(simple, &mut stack);

        let arg = switch(&mut ctx, 10);
        assert_eq!(arg, 11);

        // continue execution
        switch(&mut ctx, 0);
        // stack should be empty to this point
        assert_eq!(ctx.get_sp().cast(), stack.base());
    }
}

#[test]
fn ping_pong_multiple_empty() {
    unsafe {
        let mut stack = Stack::new(1 << 10);
        let mut ctx = it(
            |mut arg, mut ctx| loop {
                arg = switch(&mut ctx, arg - 1);
            },
            &mut stack,
        );
        const N: usize = 1 << 10;

        let mut i = N;
        while i > 0 {
            let j = i;
            i = switch(&mut ctx, i);
            assert_eq!(i, j - 1);
        }
        assert_eq!(i, 0);
    }
}

#[test]
fn ping_pong_with_workload() {
    fn simple(arg: usize, mut ctx: Context) -> Context {
        let mut v = Vec::new();
        for i in 0..arg {
            v.push(i * arg);
        }
        unsafe { switch(&mut ctx, v.len() + 1) };
        ctx
    }

    unsafe {
        let mut stack = Stack::new(1 << 20);
        let mut ctx = it(simple, &mut stack);

        let arg = switch(&mut ctx, 10);
        assert_eq!(arg, 11);

        assert!(ctx.get_sp().cast() < stack.base());
        // return back into the functor to let it run to completion and free it's resources
        switch(&mut ctx, 0);
        assert_eq!(ctx.get_sp().cast(), stack.base());
    }
}

#[test]
fn ping_pong_multiple_leak() {
    fn simple(_: usize, mut ctx: Context) -> Context {
        let mut v = Vec::new();
        loop {
            unsafe { v.push(switch(&mut ctx, v.len())) };
        }
    }

    unsafe {
        let mut stack = Stack::new(1 << 20);
        let mut ctx = it(simple, &mut stack);

        let mut i = 0;
        // big enough to trigger multiple allocations
        const N: usize = 10 << 20;
        let mut v = vec![];

        while i < N {
            i = switch(&mut ctx, i);
            v.push(i);
        }

        let arg = switch(&mut ctx, 0);
        assert_eq!(arg, N + 1);
        assert_eq!(v.len(), arg);
        // functor's `Vec` haven't being freed and we leacked here, nice
    }
}

#[test]
fn ping_pong_multiple_with_simd_workload() {
    const N: usize = 1 << 10;

    fn simple(mut arg: usize, mut ctx: Context) -> Context {
        println!("{arg}");
        eprintln!("{arg}");
        // `hashbrown` using simd for lookup operations, use is as a workload
        let mut m = HashMap::new();
        unsafe {
            while m.len() < N {
                m.insert(arg, arg);
                arg = switch(&mut ctx, m.len());
            }
        }
        assert_eq!(m.len(), N);
        ctx
    }

    unsafe {
        let mut stack = Stack::new(10 << 20);
        let mut ctx = it(simple, &mut stack);

        let mut i = 0;

        while i < N {
            i = switch(&mut ctx, i);
        }
        switch(&mut ctx, 0);

        assert_eq!(ctx.get_sp().cast(), stack.base());
    }
}

#[test]
fn ping_pong_multiple_with_simd_happy_leak() {
    const N: usize = 1 << 10;

    fn simple(mut arg: usize, mut ctx: Context) -> Context {
        println!("{arg}");
        eprintln!("{arg}");
        let mut m = HashMap::new();
        unsafe {
            while m.len() < N {
                m.insert(arg, arg);
                arg = switch(&mut ctx, m.len());
            }
        }
        assert_eq!(m.len(), N);
        ctx
    }

    unsafe {
        let mut stack = Stack::new(10 << 20);
        let mut ctx = it(simple, &mut stack);

        let mut i = 0;

        while i < N {
            i = switch(&mut ctx, i);
        }

        assert!(ctx.get_sp().cast() < stack.base());
    }
}

/*#[test]
#[should_panic]
fn should_panic() {
    fn t(arg: usize, ctx: Context) -> Context {
        panic!("hello")
    }

    unsafe {
        let mut stack = Stack::new(1 << 20);
        let mut c = it(t, &mut stack);
        _ = switch(&mut c, 0);
    }
}*/
