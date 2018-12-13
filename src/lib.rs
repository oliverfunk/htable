#[derive(Debug, Clone)]
pub enum HTableItems {
    IntegerItem(i64),
    FloatItem(f64),
    FieldItem(String),
    BooleanItem(bool),
}

#[derive(Debug)]
pub struct HTableData<'a> {
    items: Vec<&'a HTableItems>,
}

impl<'a> HTableData<'a> {
    pub fn new(items: Vec<&'a HTableItems>) -> Self {
        HTableData {
            items
        }
    }

    /// Get the items as an owned variable
    pub fn get_items(self) -> Vec<&'a HTableItems> {
        self.items
    }
}


#[derive(Debug)]
pub struct IterHTableData<'a> {
    curr_idx: usize,
    items: Vec<&'a HTableItems>,
}

impl<'a> IterHTableData<'a> {
    pub fn new(items: Vec<&'a HTableItems>) -> Self {
        IterHTableData {
            curr_idx: 0,
            items
        }
    }
}

impl<'a> Iterator for IterHTableData<'a> {
    type Item = &'a HTableItems;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.curr_idx >= self.items.len(){
            None
        } else {
            self.curr_idx += 1;
            Some(self.items[self.curr_idx - 1])
        }
    }
}

#[derive(Debug)]
pub struct HTable(Vec<Vec<HTableItems>>);

impl HTable {
    pub fn new() -> Self {
        HTable(Vec::new())
    }

    pub fn append_row(&mut self, row: Vec<HTableItems>) {
        if self.0.len() == 0 {
            // table is uninitialised
            for row_data in row {
                self.0.push(vec![row_data]);
            }
        } else if self.0.len() != row.len() {
            panic!("The number of columns in the row you are trying to add does not match the number of columns in the table")
        } else {
            for col_idx in 0..row.len() {
                let row_result = &row[col_idx];
                let table_col = &mut self.0[col_idx];

                match (row_result, table_col.as_slice()) {
                    (HTableItems::IntegerItem(_), [HTableItems::IntegerItem(_)])
                    | (HTableItems::FloatItem(_), [HTableItems::FloatItem(_)])
                    | (HTableItems::FieldItem(_), [HTableItems::FieldItem(_)])
                    | (HTableItems::BooleanItem(_), [HTableItems::BooleanItem(_)]) => {
                        table_col.push(row_result.clone());
                    }
                    _ => panic!("Mismatched types between row and table"),
                }
            }
        }
    }

    pub fn number_of_columns(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn get_row(&self, row: usize) -> Option<HTableData> {
        let mut row_data = Vec::new();

        if row < self.number_of_rows() {
            for c_inx in 0..self.number_of_columns() {
                row_data.push(&self.0[c_inx][row]);
            }

            Some(HTableData::new(row_data))
        }else{
            None
        }
    }

    pub fn get_col(&self, col: usize) -> Option<HTableData> {
        let mut col_data = Vec::new();

        if col < self.number_of_columns() {
            for r_idx in 0..self.number_of_rows() {
                col_data.push(&self.0[col][r_idx]);
            }

            Some(HTableData::new(col_data))
        }else{
            None
        }
    }

    pub fn iter_row(&self, row: usize) -> Option<impl Iterator<Item=&HTableItems>> {
        let mut row_data = Vec::new();

        if row < self.number_of_rows() {
            for c_inx in 0..self.number_of_columns() {
                row_data.push(&self.0[c_inx][row]);
            }

            Some(IterHTableData::new(row_data))
        }else{
            None
        }
    }

    pub fn iter_col(&self, col: usize) -> Option<impl Iterator<Item=&HTableItems>> {
        let mut col_data = Vec::new();

        if col < self.number_of_columns() {
            for r_idx in 0..self.number_of_rows() {
                col_data.push(&self.0[col][r_idx]);
            }

            Some(IterHTableData::new(col_data))
        }else{
            None
        }
    }
}

// todo: write tests!
