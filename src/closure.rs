fn main() {
    fn f1() -> Box<FnMut(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    fn f2() -> Box<Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    // fn f3() -> Box<FnOnce(i32) -> i32> {
    //     let num = 5;
    //     Box::new(move |x| {x + num})
    // }

    let mut fn1 = f1();
    // assert_eq!(11, fn1(6));
    // let fn2 = f2();
    let q = 6i32;
    assert_eq!(11, fn1(q));

    // let fn3 = f3();
    // assert_eq!(11, fn3(6));


    fn do_twice<F>(mut func: F)
        where F: Fn()
    {
        func();
        func();
    }

    let mut x: usize = 1;
    //{
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    //}

    assert_eq!(x, 5);
}