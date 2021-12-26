use core::fmt::{Debug, Display};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
type Link<K, V> = Option<Box<Entry<K, V>>>;

#[derive(Debug, Default, Clone)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
    //hash : usize,
    pub next: Link<K, V>,
}

#[derive(Debug)]
pub struct Hashtable<K, V> {
    pub table: Vec<Option<Box<Entry<K, V>>>>,
    pub count: usize,
    pub threshold: usize,
    pub load_factor: f32,
    pub mod_count: u64,
}

impl<K, V> Entry<K, V>
where
    K: Debug + Display + Hash + Eq + Clone,
    V: Debug + Display + Clone,
{
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.key.hash(&mut hasher);
        hasher.finish()
    }
    pub fn print(&self) {
        //println!("{}", self.key);
        //println!("{:?}", self.next.as_ref().unwrap().next.as_ref().unwrap().value);
        println!("{:?}", self.hash());
    }
}
impl<K, V> Hashtable<K, V>
where
    K: Debug + Display + Hash + Eq + Clone,
    V: Debug + Display + Clone,
{
    pub fn new() -> Self {
        Self {
            table: vec![None; 11],
            count: 0,
            load_factor: 0.75,
            threshold: ((0.75 * 11.0) as usize),
            mod_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_entries_estructure() {
        let val: Entry<_, _> = Entry {
            //hash: 123,
            key: 12312,
            value: "sdf",
            next: Some(Box::new(Entry {
                value: "1",
                //hash: 123,
                key: 90,
                next: Some(Box::new(Entry {
                    value: "2",
                    //hash: 123,
                    key: 180,
                    next: Some(Box::new(Entry {
                        value: "3",
                        next: None,
                        //hash: 123,
                        key: 0,
                    })),
                })),
            })),
        };
        let val2: Entry<_, _> = Entry {
            //hash: 123,
            key: 12312,
            value: "sdf",
            next: Some(Box::new(Entry {
                value: "1",
                //hash: 123,
                key: 90,
                next: Some(Box::new(Entry {
                    value: "2",
                    //hash: 123,
                    key: 180,
                    next: Some(Box::new(Entry {
                        value: "3",
                        next: None,
                        //hash: 123,
                        key: 0,
                    })),
                })),
            })),
        };
        assert_eq!(val2.next.as_ref().unwrap().value, "1");
    }
    #[test]
    fn create_hashtable() {
        let mut a: Hashtable<i32, &str> = Hashtable::new();
        a.table.push(Some(Box::new(val2)));
    }

}
