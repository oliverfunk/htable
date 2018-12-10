extern crate htable;

use htable::*;

fn main() {
    let mut ht = HTable::new();
    let mut test = HTableRow::new(vec![
        HTableTypes::Integer(10),
        HTableTypes::Integer(11),
        HTableTypes::Field("oli".to_string()),
    ]);
    test.set_row_data(vec![
        HTableTypes::Integer(10),
        HTableTypes::Integer(11),
        HTableTypes::Field("oliVar".to_string()),
    ]);
    let test2 = HTableRow::new(vec![
        HTableTypes::Integer(10),
        HTableTypes::Integer(11),
        HTableTypes::Field("is the best".to_string()),
    ]);
    ht.append_row(test);
    ht.append_row(test2);
    println!("{:?}", ht);
}
