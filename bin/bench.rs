use std::time::Instant;
use trailit::{it, stack::Stack, switch, Context};

const N: usize = 300_000_000;

fn predictable(mut arg: usize, mut ctx: Context) -> Context {
    unsafe {
        while arg > 1 {
            arg = switch(&mut ctx, arg - 1);
        }

        switch(&mut ctx, arg - 1);
    }
    ctx
}

#[rustfmt::skip]
fn unpredictable(mut arg: usize, mut ctx: Context) -> Context {
    macro_rules! o {() => { arg = switch(&mut ctx, arg - 1) }}
    unsafe { loop {
    // this 'rectangle' ensures CPU jump predictor having a good time
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();o!();
}}}

#[inline(never)]
fn decrement(arg: usize) -> usize {
    arg - 1
}

#[allow(unused_assignments)]
fn main() {
    core_affinity::set_for_current(core_affinity::get_core_ids().unwrap()[0]);

    unsafe {
        let mut i;

        macro_rules! timeit {
            ($b:block) => {
                {
                i = N;
                let start = Instant::now();
                $b
                start.elapsed().as_nanos() as f64 / N as f64
            }};
        }

        let fn_ = timeit!({
            while i > 0 {
                i = decrement(i);
            }
        });
        let null_ = timeit!({
            for _ in 0..N {
                std::arch::asm!("");
            }
        });

        let mut stack = Stack::new(2 << 20);
        let mut ctx = it(predictable, &mut stack);
        let switch_pred = timeit!({
            while i > 0 {
                i = switch(&mut ctx, i);
            }
        });

        let mut ctx = it(unpredictable, &mut stack);
        let switch_unpred = timeit!({
            while i > 0 {
                i = switch(&mut ctx, i);
            }
        });

        println!("    null: {null_:#.2} ns");
        println!(" fn call: {fn_:#.2} ns");
        println!("  switch: {switch_pred:#.2} ns");
        println!(" switch*: {switch_unpred:#.2} ns");
    }
}
