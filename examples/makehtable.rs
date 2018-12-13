extern crate htable;

use htable::*;

fn main() {
    let mut ht = HTable::new();
    let test = vec![
        HTableItems::IntegerItem(1),
        HTableItems::IntegerItem(11),
        HTableItems::FieldItem("oli".to_string()),
    ];
    let test2 = vec![
        HTableItems::IntegerItem(2),
        HTableItems::IntegerItem(12),
        HTableItems::FieldItem("is the best".to_string()),
    ];
    ht.append_row(test);
    ht.append_row(test2);
    println!("{:?}", ht);

    println!("\nIteration through 3rd col:");
    for ci in ht.iter_col(2).unwrap() {
        println!("{:?}", ci);
    }

    println!("\nIteration through 2nd row:");
    for ri in ht.iter_row(1).unwrap() {
        println!("{:?}", ri);
    }
}
