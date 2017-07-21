fn p2(_: i32, _:i32) {
}

fn c1() -> Result<(), i32> {
    let a = Ok(1);
    let b = a?;
    p2(123);

    Ok(())
}