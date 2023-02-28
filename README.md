An experiment with continuations on x86_64 CPU in Rust.

Minimum viable implementation of the continuation-style context switching in inline assembly, with
compiler-assisted register clobbering and function inlining. 

```rust
use trailit::{it, stack::Stack, switch, Context};

fn simple(mut arg: usize, mut ctx: Context) -> Context {
	loop {
	    arg = unsafe { switch(&mut ctx, arg - 1) };
	}
}

let mut stack = Stack::new(1 << 10);
let mut ctx = unsafe{ it(simple, &mut stack) };
let mut i = 10 << 20;

while i > 0{
	i = unsafe{ switch(&mut ctx, i) };
}
```

**performance:**

switch CPU context to a cached predictable function and return back to the caller, overall simulating a function call
> switch: 1.75ns, 0.88ns per switch

for comparison:
> noinline function call: 1.19ns

> empty asm block: 0.24ns

**limitations:**
- stateless closures only
- no stack unwinding nor debuggers support
- no continuation stopping, thus switched-to function should run to completion to release it's 
heap-allocated memory
- rust nightly
- yet to be discovered



