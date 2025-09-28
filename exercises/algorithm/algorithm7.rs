/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    
    fn len(&self) -> usize {
        self.size
    }
    
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }
    
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }
    
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { 
            stack: Vec::new() 
        };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { 
            stack: Vec::new() 
        };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}')];
    
    for ch in bracket.chars() {
        match ch {
            // 如果是左括号，压入栈中
            '(' | '[' | '{' => {
                stack.push(ch);
            },
            // 如果是右括号，检查是否匹配
            ')' | ']' | '}' => {
                if let Some(top) = stack.pop() {
                    // 检查栈顶的左括号是否与当前右括号匹配
                    let is_match = bracket_pairs.iter().any(|&(left, right)| {
                        top == left && ch == right
                    });
                    if !is_match {
                        return false;
                    }
                } else {
                    // 栈为空但遇到右括号，不匹配
                    return false;
                }
            },
            // 非括号字符直接跳过
            _ => continue,
        }
    }
    
    // 所有字符处理完后，栈应为空
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bracket_matching_1(){
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_2(){
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_3(){
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_4(){
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_5(){
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_6(){
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}