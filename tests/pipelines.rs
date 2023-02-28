use trailit::{it, stack::Stack, switch, Context};
#[test]
fn centralized() {
    // main -> c1
    // main -> c2
    // main -> c3
    fn c(mut arg: usize, mut ctx: Context) -> Context {
        loop {
            arg = unsafe { switch(&mut ctx, arg + 1) };
        }
    }
    let mut s1 = Stack::new(1 << 10);
    let mut s2 = Stack::new(1 << 10);
    let mut s3 = Stack::new(1 << 10);
    unsafe {
        let mut c1 = it(c, &mut s1);
        let mut c2 = it(c, &mut s2);
        let mut c3 = it(c, &mut s3);
        assert_eq!(switch(&mut c1, 0), 1);
        assert_eq!(switch(&mut c2, 1), 2);
        assert_eq!(switch(&mut c3, 2), 3);

        assert_eq!(switch(&mut c1, 3), 4);
        assert_eq!(switch(&mut c2, 4), 5);
        assert_eq!(switch(&mut c3, 5), 6);
    }
}

#[test]
fn back_and_forth() {
    //  c1 -> c2 -> c3  ┐
    //  c1 <- c2 <- c3 <┘
    unsafe {
        fn c1(mut arg: usize, mut ctx: Context) -> Context {
            unsafe {
                let mut stack = Stack::new(1 << 10);
                let mut c2 = it(c2, &mut stack);
                loop {
                    arg = switch(&mut c2, arg + 1);
                    arg = switch(&mut ctx, arg);
                }
            }
        }

        fn c2(mut arg: usize, mut ctx: Context) -> Context {
            unsafe {
                let mut stack = Stack::new(1 << 10);
                let mut c3 = it(c3, &mut stack);
                loop {
                    arg = switch(&mut c3, arg + 10);
                    arg = switch(&mut ctx, arg);
                }
            }
        }

        fn c3(mut arg: usize, mut ctx: Context) -> Context {
            unsafe {
                loop {
                    arg = switch(&mut ctx, arg + 100);
                }
            }
        }

        let mut stack = Stack::new(1 << 10);
        let mut c = it(c1, &mut stack);
        let arg = switch(&mut c, 0);
        assert_eq!(arg, 111);
        let arg = switch(&mut c, 1000);
        assert_eq!(arg, 1111);
        let arg = switch(&mut c, 2111);
        assert_eq!(arg, 2222);
    }
}
