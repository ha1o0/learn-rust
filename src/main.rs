use learn_rust::base_type_4_1;
use learn_rust::variable_3;

fn main() {
    variable_3::test();
    base_type_4_1::test();
    println!("Hello, world!");
    hello();
    let a: f32 = 12.62;
    println!("round: {}", a.round());
    println!("x: {}", func());
    copy1();
    yinyong();
    _strtest();
    tupletest();
    structtest();
    println!("add: {}", add(3, 5));
    let t = Test { a: 5, b: "halob" };
    println!("AB: {:?}", t.get());
    let b = [1, 2, 5];
    printarr(&b);
    printarr_t(b);
    testvector();
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn testvector() {
    let a = vec![
        IpAddr::V4(String::from("8.8.8.8")),
        IpAddr::V6(String::from("aa::bb::cc::dd")),
    ];
    for item in a {
        showip(item);
    }
}

fn showip(ip: IpAddr) {
    println!("ip: {:?}", ip);
}

fn printarr(arr: &[i32]) {
    println!("arr: {:?}", arr);
}

fn printarr_t<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("arr: {:?}", arr);
}

#[derive(Debug)]
struct Test<T, P> {
    a: T,
    b: P,
}

impl<T, P> Test<T, P> {
    fn get(&self) -> (&T, &P) {
        (&self.a, &self.b)
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn copy1() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

fn func() -> i32 {
    let mut x = 33;
    x = x - 8;
    x
}

fn yinyong() {
    let a_string: String = String::from("helloo");
    let a_yin = &a_string;
    println!(
        "avalue: {}, yin value: {}, jieyin value: {}",
        a_string, a_yin, *a_yin
    );
    println!("len: {}", a_yin.len());
}

fn _strtest() {
    let mut a = String::from("中国人");
    a.insert(3, 'o');
    println!("a value: {}", a);
}

fn tupletest() {
    let a = (1, "aa", false, 3.4);
    println!("a value: {}", a.2);
}

#[derive(Debug)]
struct User {
    name: String,
    age: i8,
}

fn structtest() {
    let mut user = User {
        name: String::from("joe"),
        age: 23,
    };
    user.name = "hell".to_string();
    user.age = 24;
    dbg!(&user);
    println!("{:?}", user);
}

// fn returnv() -> i32 {
//     let y = {
//         let x = 3;
//         x * 2
//     };
//     y
// }

fn hello() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "world, hello";
    let regions = [southern_germany, chinese, english];
    // for region in regions {
    //     println!("{}", region);
    // }
    for region in regions.iter() {
        println!("{}", &region);
    }
}
