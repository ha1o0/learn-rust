pub fn test() {
    a();
    b();
    c();
    d();
}

// 移除某个部分让代码工作
fn a() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let _z = 10; // 这里 z 的类型是?
}

// 填空
fn b() {
    let _v: u16 = 38_u8 as u16;
}

//  修改 `assert_eq!` 让代码工作
fn c() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// 填空，让代码工作
fn d() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}
