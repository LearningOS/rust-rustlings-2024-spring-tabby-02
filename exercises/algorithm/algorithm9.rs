/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default+ std::cmp::PartialOrd,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+ std::cmp::PartialOrd,
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

    pub fn add(&mut self, value: T) {
        if self.count==0{self.items[0]=value;}
        else{self.items.push(value);}
        self.count+=1;
        let mut idx=self.count;
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if parent_idx==0{break;}
            if (self.comparator)(&self.items[parent_idx-1], &self.items[idx-1] ){
                break;
            }
            self.items.swap(parent_idx-1, idx-1);
            idx = parent_idx;
        }
        //TODO
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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let mut id:usize =0;
        for i in 1..self.count{
            if self.items[i]<self.items[i-1]{
                id=i;
            }
        }
		id
    }
}

impl<T> Heap<T>
where
    T: Default + Ord+ std::cmp::PartialOrd,
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
    T: Default+ std::cmp::PartialOrd+ Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count==0 {
            return None;
        }
        let result = self.items[0].clone();
        // let p=self.items[0].clone();
		for i in 0..self.count-1{
			self.items[i]=self.items[i+1].clone();
		}
        self.items.pop();
        self.count-=1;
        let last_index = self.count - 1;
        self.items.swap(0, last_index);
        let len = self.count;
        let mut idx=1;
        loop {
            let left_child_idx= self.left_child_idx(idx);
            let right_child_idx = self.right_child_idx(idx);
            let mut smallest_idx = idx;

            // if (self.comparator)(&left_child_idx, &(len+1)) && (self.comparator)(&self.items[left_child_idx-1] , &self.items[smallest_idx-1] ){
            //     smallest_idx = left_child_idx;
            // }

            // if (self.comparator)(&right_child_idx , &(len+1)) && (self.comparator)(&self.items[right_child_idx-1], &self.items[smallest_idx-1] ){
            //     smallest_idx = right_child_idx;
            // }
            

            if left_child_idx<len+1 && (self.comparator)(&self.items[left_child_idx-1] , &self.items[smallest_idx-1] )
            {
                smallest_idx = left_child_idx;
            }

            if right_child_idx<len+1 && (self.comparator)(&self.items[right_child_idx-1], &self.items[smallest_idx-1] )
            {
                smallest_idx = right_child_idx;
            }
            
            if smallest_idx == idx {
                break;
            }

            self.items.swap(idx-1, smallest_idx-1);
            idx = smallest_idx;
        }

        //TODO
		Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::cmp::PartialOrd,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::cmp::PartialOrd,
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
        // println!("{}",heap.items.to_string());
        // heap.items.iter().for_each(|item| println!("{}", item));
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        // println!("///////");
        // heap.items.iter().for_each(|item| println!("{}", item));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        // println!("///////");
        // heap.items.iter().for_each(|item| println!("{}", item));
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.items.iter().for_each(|item| println!("{}", item));
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}