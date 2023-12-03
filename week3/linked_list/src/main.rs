use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // test generics for String
    let mut list2: LinkedList<String> = LinkedList::new();
    assert!(list2.is_empty());
    assert_eq!(list2.get_size(), 0);
    list2.push_front(String::from("a"));
    list2.push_front(String::from("b"));
    list2.push_front(String::from("c"));
    println!("{}", list2.to_string());

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, ComputeNorm};
    #[test]
    fn test_generics() {
        let mut list: LinkedList<String> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.get_size(), 0);

        list.push_front("a".to_string());
        list.push_front("b".to_string());
        list.push_front("c".to_string());
    
        assert_eq!(" c b a", list.to_string());
        assert_eq!(3, list.get_size());
        assert_eq!("c", list.pop_front().unwrap());
        assert_eq!(" b a", list.to_string());
        assert_eq!(2, list.get_size());
    }

    #[test]
    fn test_clone() {
        let mut li = LinkedList::new();
        li.push_front(3);
        li.push_front(2);
        li.push_front(1);

        let mut li2 = li.clone();
        li.push_front(0);
        li2.push_front(4);
        assert_eq!(" 0 1 2 3", li.to_string());
        assert_eq!(" 4 1 2 3", li2.to_string());
    }

    #[test]
    fn test_partialeq() {
        let mut li = LinkedList::new();
        li.push_front(3);
        li.push_front(2);
        li.push_front(1);
        let li2 = li.clone();
        assert!(li2 == li);
    }

    #[test]
    fn test_computenorm() {
        let mut li = LinkedList::new();
        li.push_front(3.0);
        li.push_front(4.0);
        assert_eq!(5.0, li.norm());
    }

    #[test]
    fn test_iterator() {
        let mut li = LinkedList::new();
        li.push_front(0);
        li.push_front(1);
        li.push_front(2);
        li.push_front(3);
        let mut j = 3;
        for i in li {
            assert!(j == i);
            j -= 1;
        }
    }

    #[test]
    fn test_into_iterator() {
        let mut li = LinkedList::new();
        li.push_front(0);
        li.push_front(1);
        li.push_front(2);
        li.push_front(3);
        let mut j = 3;
        for i in li.into_iter() {
            assert_eq!(j, i);
            j -= 1;
        }
    }

}
