// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }  // 处理 Option 类型时，可以使用模式匹配或简便的方法（如 if let 或者 unwrap_or），而不是通过循环的方式。
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
