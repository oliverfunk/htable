#[derive(Debug, Clone)]
pub enum HTableTypes {
    Integer(i64),
    Float(f64),
    Field(String),
    Boolean(bool),
}

pub struct HTable (Vec<Vec<HTableTypes>>);

impl HTable {
    pub fn new (type_sig: Vec<HTableTypes>) -> Self {
        let mut result_set = Vec::new();

        for t in &type_sig {
            let mut col = Vec::new();
            col.push(t.clone());
            col.pop();
            result_set.push(col);


//            match t {
//                &HTableTypes::Integer(i) => {
//                    let mut col = Vec::new();
//                    col.push(HTableTypes::Integer(i));
//
//                    result_set.push(col);
//                }
//                _ => {
//                    let mut col = Vec::new();
//                    col.push(HTableTypes::Integer(0));
//
//                    result_set.push(col)
//                }
//            }
        }



        HTable(result_set)
    }

    pub fn append_row(&mut self, row: Vec<HTableTypes>) {

    }
}

// todo: write tests!