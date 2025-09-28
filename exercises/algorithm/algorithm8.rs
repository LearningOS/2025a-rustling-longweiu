/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


use std::mem;

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { elements: Vec::new() }
    }
    
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }
    
    // 修复：使用'static生命周期
    pub fn dequeue(&mut self) -> Result<T, &'static str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }
    
    pub fn size(&self) -> usize {
        self.elements.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }
    
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        
        // 将q1中除最后一个元素外的所有元素转移到q2
        while self.q1.size() > 1 {
            if let Ok(elem) = self.q1.dequeue() {
                self.q2.enqueue(elem);
            }
        }
        
        // 弹出最后一个元素（栈顶）
        // 修复：现在dequeue返回'static错误，不会持有借用
        let popped_elem = self.q1.dequeue()?;
        
        // 交换q1和q2的角色
        mem::swap(&mut self.q1, &mut self.q2);
        
        Ok(popped_elem)
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack(){  // 修改函数名更准确（可选）
        let mut s = MyStack::<i32>::new();  // 关键修正：MyStack 而非 myStack
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}