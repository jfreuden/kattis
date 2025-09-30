pub type IndexType = usize;
pub type ValueType = i64;

pub struct FenwickTree {
    data_fenwick: Vec<ValueType>,
}

impl FenwickTree {
    pub fn new(array_len: usize) -> Self {
        FenwickTree {
            data_fenwick: vec![0 as ValueType; array_len],
        }
    }

    pub fn increment(&mut self, index: usize, value: ValueType) {
        let max_index = self.data_fenwick.len();
        let mut working_index = index as ValueType + 1;

        while working_index <= max_index as ValueType {
            self.data_fenwick[working_index as usize - 1] += value;
            working_index = working_index + (working_index & -working_index);
        }
    }

    pub fn query(&self, index: usize) -> ValueType {
        let query_index = index;
        if query_index == 0 {
            0
        } else {
            let mut sum = 0;

            let mut working_index = query_index as ValueType;
            while working_index > 0 {
                sum += self.data_fenwick[working_index as usize - 1];
                working_index = working_index - (working_index & (-working_index));
            }
            sum
        }
    }
}