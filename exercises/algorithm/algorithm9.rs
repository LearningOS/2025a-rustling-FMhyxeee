/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加新元素
    /// 向堆中插入一个元素，并执行上浮以维持堆序
    pub fn add(&mut self, value: T) {
        // 先增加计数，逻辑位置为 self.count
        self.count += 1;

        // 若物理数组长度不足，push；否则复用已存在的空槽
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }

        // 上浮：比较当前结点与父结点，若更优则交换
        let mut idx = self.count;
        while idx > 1 {
            let p = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[p]) {
                self.items.swap(idx, p);
                idx = p;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 找到给定节点的最小子节点索引
    /// 根据比较器函数确定哪个子节点更符合堆性质
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 只有左孩子
        if right > self.count {
            return left;
        }
        // 同时存在左右孩子，返回更优的那个
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }

    /// 下沉操作：将指定位置的元素向下调整到正确位置
    fn heapify_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            
            // 如果当前元素已经满足堆性质，停止下沉
            if !(self.comparator)(&self.items[child_idx], &self.items[idx]) {
                break;
            }
            
            // 交换当前元素与子元素
            self.items.swap(idx, child_idx);
            idx = child_idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    /// 从堆中取出根元素（最小值或最大值）
    /// 取出后重新调整堆结构维护堆性质
    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        // 取出根元素
        let root = std::mem::replace(&mut self.items[1], T::default());
        
        if self.count == 1 {
            // 仅有一个有效元素时，移除占位的 index 1，使得仅保留 items[0] 哨兵
            self.count = 0;
            let _ = self.items.pop();
            return Some(root);
        }
        
        // 用末尾元素覆盖根，再弹出末尾，保证物理长度与逻辑一致
        let last = self.items.pop().unwrap();
        self.items[1] = last;
        self.count -= 1;
        
        // 下沉操作，重新调整堆
        self.heapify_down(1);
        
        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}