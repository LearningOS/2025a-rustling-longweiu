/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self 
where
    T: PartialOrd,
{
    let mut merged_list = LinkedList::new();
    
    // 同时处理两个链表，直到其中一个为空
    while list_a.start.is_some() && list_b.start.is_some() {
        unsafe {
            let a_val = &(*list_a.start.unwrap().as_ptr()).val;
            let b_val = &(*list_b.start.unwrap().as_ptr()).val;
            
            if a_val <= b_val {
                // 从list_a取出节点
                let node_ptr = list_a.start.take().unwrap();
                list_a.start = (*node_ptr.as_ptr()).next;
                
                // 如果取出的节点是最后一个节点，需要更新end指针
                if list_a.start.is_none() {
                    list_a.end = None;
                }
                
                // 将节点添加到合并后的链表
                Self::add_node_to_list(&mut merged_list, node_ptr);
            } else {
                // 从list_b取出节点
                let node_ptr = list_b.start.take().unwrap();
                list_b.start = (*node_ptr.as_ptr()).next;
                
                // 如果取出的节点是最后一个节点，需要更新end指针
                if list_b.start.is_none() {
                    list_b.end = None;
                }
                
                // 将节点添加到合并后的链表
                Self::add_node_to_list(&mut merged_list, node_ptr);
            }
        }
    }
    
    // 处理list_a剩余的节点 - 直接连接整个剩余链表
    if list_a.start.is_some() {
        if merged_list.end.is_some() {
            unsafe {
                // 将合并链表的末尾连接到list_a的剩余部分
                (*merged_list.end.unwrap().as_ptr()).next = list_a.start;
            }
            merged_list.end = list_a.end;
        } else {
            // 如果合并链表为空，直接使用list_a的剩余部分
            merged_list.start = list_a.start;
            merged_list.end = list_a.end;
        }
        merged_list.length += list_a.length;
    }
    
    // 处理list_b剩余的节点 - 直接连接整个剩余链表
    if list_b.start.is_some() {
        if merged_list.end.is_some() {
            unsafe {
                // 将合并链表的末尾连接到list_b的剩余部分
                (*merged_list.end.unwrap().as_ptr()).next = list_b.start;
            }
            merged_list.end = list_b.end;
        } else {
            // 如果合并链表为空，直接使用list_b的剩余部分
            merged_list.start = list_b.start;
            merged_list.end = list_b.end;
        }
        merged_list.length += list_b.length;
    }
    
    merged_list
}}
impl<U> LinkedList<U> {
// 辅助函数：将节点添加到链表末尾
fn add_node_to_list<T>(list: &mut LinkedList<T>, mut node_ptr: NonNull<Node<T>>) {
    unsafe {
        // 确保新节点的next为None，因为它将成为新链表的最后一个节点
        (*node_ptr.as_ptr()).next = None;
    }
    
    match list.end {
        None => {
            // 如果链表为空，新节点既是头也是尾
            list.start = Some(node_ptr);
            list.end = Some(node_ptr);
        },
        Some(end_ptr) => unsafe {
            // 将新节点连接到当前链表的末尾
            (*end_ptr.as_ptr()).next = Some(node_ptr);
            list.end = Some(node_ptr);
        },
    }
    list.length += 1;
}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}