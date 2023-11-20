pub fn exec() {
    a();
}

fn a() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("matrix:");
    pretty_print(&matrix);

    let matrix2 = vec![
        vec!["101", "102", "103"],
        vec!["201", "202", "203"],
        vec!["301", "302", "303"],
    ];

    let transposed = transpose(matrix2);
    println!("transposed:");
    // println!("{:?}", transposed);
    pretty_print2(&transposed);

    let ipv4 = Ip::Ipv4(String::from("127.0.0.1"));
    let ipv6 = Ip::Ipv4(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("{:?}, {:?}", ipv4, ipv6);

    let bar_a = Bar::A;
    let bar_b = Bar::B;
    println!("{:?}, {:?}", bar_a, bar_b);

    let result = luhn("7992739871");
    println!("{result}");
}

#[allow(unused_variables, dead_code)]
fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.len() == 0 {
        return vec![];
    }
    let rows = matrix.len();
    let columns = matrix[0].len();
    let mut transpose_matrix = vec![vec![matrix[0][0].clone(); columns]; rows];
    for i in 0..rows {
        for j in 0..columns {
            transpose_matrix[j][i] = matrix[i][j].clone();
        }
    }
    transpose_matrix
}

#[allow(unused_variables, dead_code)]
fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for arr in *matrix {
        for n in &arr {
            print!("{n} ");
        }
        println!();
    }
}

fn pretty_print2<T: Clone + std::fmt::Debug>(matrix: &Vec<Vec<T>>) {
    let rows = matrix.len();
    let columns = matrix[0].len();
    for i in 0..rows {
        for j in 0..columns {
            print!("{:?} ", matrix[i][j]);
        }
        println!();
    }
}

#[derive(Debug)]
enum Ip {
    Ipv4(String),
    Ipv6(String),
}

#[derive(Debug)]
enum Bar {
    A = 10,
    B = 11,
}

fn luhn(number: &str) -> bool {
    let target_str: Vec<char> = number
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .chars()
        .collect();
    println!("{:?}", target_str);
    if target_str.len() < 2 {
        return false;
    }
    let str_len = target_str.len();
    let mut sum = 0;
    for i in 0..str_len {
        let j = str_len - 1 - i;
        let item = target_str[j].to_digit(10);
        if item.is_none() {
            println!("is none");
            return false;
        }
        let mut num = item.unwrap();
        if num > 9 {
            println!("> 9");
            return false;
        }
        println!("index: {i}, origin: {num}");
        if i % 2 == 1 {
            num = num * 2;
            if num > 9 {
                num -= 9;
            }
        }
        println!("{num}");
        sum += num;
    }
    println!("{sum}");
    if sum % 10 != 0 {
        println!("not 0");
        return false;
    }

    true
}

#[test]
fn test_non_digit_cc_number() {
    println!("run test");
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
