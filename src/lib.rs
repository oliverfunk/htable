use std::mem;

#[derive(Debug, Clone)]
pub enum HTableTypes {
    Integer(i64),
    Float(f64),
    Field(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct HTableRow(Vec<HTableTypes>);

impl HTableRow {
    pub fn new(row_data: Vec<HTableTypes>) -> Self {
        HTableRow(row_data)
    }

    pub fn reset_row_data(&mut self, row_data: Vec<HTableTypes>) {
        self.0 = row_data;
    }

    pub fn set_row_data(&mut self, row_data: Vec<HTableTypes>) {
        if self.0.len() != row_data.len() {
            panic!("The number of columns in the row you are trying to add does not match the number of columns in the table")
        } else {
            for col_idx in 0..row_data.len() {
                let matched = {
                    let row_result = &row_data[col_idx];
                    let self_row_result = &self.0[col_idx];

                    match (row_result, self_row_result) {
                        (HTableTypes::Integer(_), HTableTypes::Integer(_))
                        | (HTableTypes::Float(_), HTableTypes::Float(_))
                        | (HTableTypes::Field(_), HTableTypes::Field(_))
                        | (HTableTypes::Boolean(_), HTableTypes::Boolean(_)) => true,
                        _ => false,
                    }
                };

                if matched {
                    mem::replace(&mut self.0[col_idx], row_data[col_idx].clone());
                } else {
                    panic!("Row types do not match.")
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct HTable(Vec<Vec<HTableTypes>>);

impl HTable {
    pub fn new() -> Self {
        HTable(Vec::new())
    }

    pub fn append_row(&mut self, row: HTableRow) {
        if self.0.len() == 0 {
            // table is uninitialised
            for row_data in row.0 {
                self.0.push(vec![row_data]);
            }
        } else if self.0.len() != row.0.len() {
            panic!("The number of columns in the row you are trying to add does not match the number of columns in the table")
        } else {
            for col_idx in 0..row.0.len() {
                let row_result = &row.0[col_idx];
                let table_col = &mut self.0[col_idx];

                match (row_result, table_col.as_slice()) {
                    (HTableTypes::Integer(_), [HTableTypes::Integer(_)])
                    | (HTableTypes::Float(_), [HTableTypes::Float(_)])
                    | (HTableTypes::Field(_), [HTableTypes::Field(_)])
                    | (HTableTypes::Boolean(_), [HTableTypes::Boolean(_)]) => {
                        table_col.push(row_result.clone());
                    }
                    _ => panic!("Mismatched types between row and table"),
                }
            }
        }
    }
}

// todo: write tests!
