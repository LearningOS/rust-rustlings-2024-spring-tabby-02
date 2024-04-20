/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


// #[derive(Debug)]
#[derive(Debug, Clone)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T:Clone> Queue<T> {
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

impl<T:Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
    len:i32,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T:Clone> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            len:0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
        self.len+=1;
        //TODO
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.q1.is_empty(){
            if self.q2.is_empty(){return Err("Stack is empty")}
            self.len-=1;
            loop{
                match self.q2.dequeue(){
                    Err(_)=>break,
                    Ok(x)=>{
                        let mut test=self.q2.clone();
                        match test.dequeue(){
                            Err(_)=>{return Ok(x);},
                            Ok(_)=>{self.q1.enqueue(x)}
                        }
                        
                    }
                }
            }
        }else{
            self.len-=1;
            loop{
                match self.q1.dequeue(){
                    Err(_)=>break,
                    Ok(x)=>{
                        let mut test=self.q1.clone();
                        match test.dequeue(){
                            Err(_)=>return Ok(x),
                            Ok(_)=>{self.q2.enqueue(x)}
                        }
                        
                    }
                }
            }

        }
		Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        0==self.len
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