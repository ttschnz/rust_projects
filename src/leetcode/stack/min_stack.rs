pub struct MinStack {
    values: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            // values: Vec::with_capacity(30000),
            values: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.values.is_empty() {
            self.values.push((val, val));
        } else {
            let min = self.values.last().unwrap().1.min(val);
            self.values.push((val, min))
        }
    }

    pub fn pop(&mut self) {
        self.values.pop();
    }

    pub fn top(&mut self) -> i32 {
        self.values.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.values.last().unwrap().1
    }
}

#[cfg(test)]
mod test {
    use super::MinStack;
    #[test]
    fn works() {
        let mut st = MinStack::new();
        st.push(-2);
        st.push(0);
        st.push(-3);
        assert_eq!(st.get_min(), -3);
        st.pop();
        assert_eq!(st.top(), 0);
        assert_eq!(st.get_min(), -2);
    }
}
