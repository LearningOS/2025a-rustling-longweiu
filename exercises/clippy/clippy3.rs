// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    use std::mem;
    
    // 1. 安全处理Option::None
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 不再调用可能panic的unwrap()
        println!("my_option is None, handling safely");
    }
    
    // 2. 修复数组语法
    let my_arr = &[
        -1, -2, -3,  // 添加了逗号
        -4, -5, -6   // 添加了逗号
    ];
    println!("My array! Here it is: {:?}", my_arr);
    
    // 3. 正确使用Vec的resize方法
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();  // 修改原Vec，而不是赋值返回值
    println!("This Vec is empty, see? {:?}", my_vec);
    
    // 4. 正确交换变量值
    let mut value_a = 45;
    let mut value_b = 66;
    mem::swap(&mut value_a, &mut value_b);  // 使用标准库函数
    println!("value a: {}; value b: {}", value_a, value_b);
}
