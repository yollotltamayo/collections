use collections::hashtable::{Entry, Hashtable};
fn main() {
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
    val.print();
    println!("{:?}", val.next.as_ref().unwrap().hash() % 5);
    println!(
        "{:?}",
        val.next.as_ref().unwrap().next.as_ref().unwrap().hash() % 7
    );
    //let tab = Hashtable::<i32, &str> {
    //  table: vec![val2, val],
    //};
    let mut a: Hashtable<i32, &str> = Hashtable::new();
    a.table.push(Some(Box::new(val2)));
}
