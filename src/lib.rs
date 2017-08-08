/*
 *  Copyright 2017 Gianmarco Garrisi
 *
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

//!This crate provide a priority queue backed by an hashmap
//!with some efficient methods to change the priority of an element in
//!**O(log(N))** time (worst case).
//!
//!The elements(called `Item`s) must implement [`Hash`]
//!(https://doc.rust-lang.org/std/hash/trait.Hash.html)
//!and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) traits.
//!
//!The priority may be any type that implements [`Ord`]
//!(https://doc.rust-lang.org/std/cmp/trait.Ord.html).
//!
//!
//!#Example
//!```rust
//!extern crate priority_queue;
//!
//!use priority_queue::PriorityQueue;
//!      
//!fn main() {
//!    let mut pq = PriorityQueue::new();
//!
//!    assert!(pq.is_empty());
//!    pq.push("Apples", 5);
//!    pq.push("Bananas", 8);
//!    pq.push("Strawberries", 23);
//!
//!    assert_eq!(pq.peek(), Some((&"Strawberries", &23)));
//!    
//!    pq.change_priority("Bananas", 25);
//!    assert_eq!(pq.peek(), Some((&"Bananas", &25)));
//!
//!    for (item, _) in pq.into_sorted_iter() {
//!        println!("{}", item);
//!    }
//!}
//!```

extern crate ordermap;

mod pqueue;
mod iterators;
pub use pqueue::PriorityQueue;

#[cfg(test)]
mod tests {
    pub use PriorityQueue;

    #[test]
    fn init(){
        let pq: PriorityQueue<String, i32> = PriorityQueue::new();
        println!("{:?}", pq);
    }

    #[test]
    fn push_len() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 2);
        println!("{:?}", pq);
        assert_eq!(pq.len(), 2);
    }

    #[test]
    fn push_pop() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 2);
        pq.push("f", 7);
        pq.push("g", 4);
        pq.push("h", 3);
        println!("{:?}", pq);
        assert_eq!(pq.pop(), Some(("f", 7)));
        println!("{:?}", pq);
        assert_eq!(pq.peek(), Some((&"g", &4)));
        assert_eq!(pq.pop(), Some(("g", 4)));
        assert_eq!(pq.len(), 3);
    }

    #[test]
    fn push_update() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 3);
        pq.push("a", 4);
        assert_eq!(pq.peek(), Some((&"a", &4)));
    }

    #[test]
    fn change_priority() {
        let mut pq = PriorityQueue::new();
        pq.push("Processor", 1);
        pq.push("Mainboard", 2);
        pq.push("Ram", 5);
        pq.change_priority("Processor", 10);
        assert_eq!(pq.peek(), Some((&"Processor", &10)));

        pq.change_priority("Ram", 11);
        assert_eq!(pq.peek(), Some((&"Ram", &11)));
    }

    #[test]
    fn from_vec() {
        let v = vec!(("a", 1), ("b", 2), ("f", 7));
        let mut pq = PriorityQueue::from(v);
        assert_eq!(pq.pop(), Some(("f", 7)));
        assert_eq!(pq.len(), 2);
    }
    
    #[test]
    fn from_vec_with_repeated() {
        let v = vec!(("a", 1), ("b", 2), ("f", 7), ("a", 2));
        let mut pq = PriorityQueue::from(v);
        assert_eq!(pq.pop(), Some(("f", 7)));
        assert_eq!(pq.len(), 2);
    }

    #[test]
    fn from_iter() {
        use std::iter::FromIterator;
        
        let v = vec!(("a", 1), ("b", 2), ("f", 7));
        let mut pq = PriorityQueue::from_iter(v.into_iter());
        assert_eq!(pq.pop(), Some(("f", 7)));
        assert_eq!(pq.len(), 2);
    }

    #[test]
    fn heap_sort() {
        let v = vec!(("a", 2), ("b", 7), ("f", 1));
        let sorted = (PriorityQueue::from(v)).into_sorted_vec();
        assert_eq!(sorted.as_slice(), &["b", "a", "f"]);
    }

    #[test]
    fn change_priority_by() {
        use std::iter::FromIterator;
        
        let v = vec!(("a", 1), ("b", 2), ("f", 7));
        let mut pq = PriorityQueue::from_iter(v.into_iter());

        pq.change_priority_by("a", |a| a+2);
        assert_eq!(pq.into_sorted_vec().as_slice(), &["f", "a", "b"]);
    }

    #[test]
    fn extend() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 2);
        pq.push("f", 7);

        let v = vec!(("c", 4), ("d", 6), ("e", 3));
        pq.extend(v);
        assert_eq!(pq.len(), 6);
        assert_eq!(pq.into_sorted_vec().as_slice(),
                   &["f", "d", "c", "e", "b", "a"]);
    }

    #[test]
    fn iter() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 2);
        pq.push("f", 7);

        assert_eq!(pq.iter().count(), 3);
    }

    #[test]
    fn into_sorted_iter(){
        let mut pq = PriorityQueue::new();
        pq.push("a", 1);
        pq.push("b", 2);
        pq.push("f", 7);

        assert_eq!(pq.into_sorted_iter().collect::<Vec<_>>(),
                   vec!(("f", 7), ("b", 2), ("a", 1)));
    }
}
