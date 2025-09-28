// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



// 使用泛型参数 T
struct Wrapper<T> {
    value: T,
}

// 为泛型类型 Wrapper<T> 实现方法
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // 编译器推断 T 为 u32
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // 编译器推断 T 为 &str
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}