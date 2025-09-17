/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    /// 创建一个新的栈实例
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new()
        }
    }
    
    /// 将元素压入栈顶
    /// 策略：总是将新元素加入到非空的队列中，如果两个队列都为空，则加入q1
    pub fn push(&mut self, elem: T) {
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q1.enqueue(elem);
        }
    }
    
    /// 从栈顶弹出元素
    /// 策略：将非空队列中除最后一个元素外的所有元素转移到另一个队列，
    /// 然后弹出最后一个元素（这就是栈顶元素）
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        
        // 确定哪个队列有数据
        if !self.q1.is_empty() {
            // q1有数据，将除最后一个元素外的所有元素转移到q2
            while self.q1.size() > 1 {
                if let Ok(elem) = self.q1.dequeue() {
                    self.q2.enqueue(elem);
                }
            }
            // 弹出最后一个元素（栈顶）
            self.q1.dequeue()
        } else {
            // q2有数据，将除最后一个元素外的所有元素转移到q1
            while self.q2.size() > 1 {
                if let Ok(elem) = self.q2.dequeue() {
                    self.q1.enqueue(elem);
                }
            }
            // 弹出最后一个元素（栈顶）
            self.q2.dequeue()
        }
    }
    
    /// 检查栈是否为空
    /// 当两个队列都为空时，栈为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
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