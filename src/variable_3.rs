pub fn test() {
    println!("hello variables");
    a();
    b();
    c();
    d();
    e();
    f();
    g();
    h();
}

// 修复下面代码的错误并尽可能少的修改
fn a() {
    let x: i32 = 123; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);
}

// 完形填空，让代码编译
fn b() {
    let mut x = 1;
    x += 2;

    println!("x = {}", x);
}

// 修复下面代码的错误并使用尽可能少的改变
fn c() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x);
}

// 修复错误
fn d() {
    println!("{}, world", define_x());
}

fn define_x<'a>() -> &'a str {
    let x = "hello";
    x
}

// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn e() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}

fn f() {
    let mut x: i32;
    x = 7;
    // 遮蔽且再次绑定
    // let mut x = x + 1;
    x += 3;

    println!("{}", x);

    let _y = 4;
    // 遮蔽
    let _y = "I can also be bound to text!";
}

// 修复下面代码的错误并尽可能少的修改
fn g() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

fn h() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x, y], [3, 2]);
}
