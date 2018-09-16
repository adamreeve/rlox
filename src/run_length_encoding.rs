pub struct RunLengthEncoded<T> {
    run_lengths: Vec<RunLength<T>>,
}

struct RunLength<T> {
    value: T,
    run_length: usize,
}

pub struct RunLengthIterator<'a, T: 'a> {
    run_length_encoded: &'a RunLengthEncoded<T>,
    index: usize,
    run_index: usize,
}

impl<T> RunLength<T> {
    pub fn new(value: T, run_length: usize) -> RunLength<T> {
        RunLength {
            value,
            run_length,
        }
    }

    pub fn increment_by(&mut self, increment: usize) {
        self.run_length += increment
    }
}

impl<T: PartialEq+Clone> RunLengthEncoded<T> {
    pub fn new() -> RunLengthEncoded<T> {
        RunLengthEncoded {
            run_lengths: Vec::new()
        }
    }

    pub fn push(&mut self, value: T) {
        self.push_run(value, 1);
    }

    pub fn push_run(&mut self, value: T, count: usize) {
        let continue_run = match self.run_lengths.last() {
            Some(run_length) if run_length.value == value => true,
            _ => false
        };
        if continue_run {
            self.run_lengths.last_mut().unwrap().increment_by(count);
        } else {
            self.run_lengths.push(RunLength::new(value, count));
        }
    }

    pub fn nth(&self, index: usize) -> T {
        self.into_iter().nth(index).unwrap().clone()
    }
}

impl<'a, T: 'a> IntoIterator for &'a RunLengthEncoded<T> {
    type Item = &'a T;
    type IntoIter = RunLengthIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RunLengthIterator {
            run_length_encoded: self,
            index: 0,
            run_index: 0
        }
    }
}

impl<'a, T: 'a> Iterator for RunLengthIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let num_run_lengths = self.run_length_encoded.run_lengths.len();
        // First get return value
        let return_value = if self.index >= num_run_lengths {
            None
        } else {
            let run_length = &self.run_length_encoded.run_lengths[self.index];
            Some(&run_length.value)
        };
        // Now mutate state for next value
        if self.index < num_run_lengths {
            let current_run_length = self.run_length_encoded.run_lengths[self.index].run_length;
            if self.run_index == current_run_length - 1 {
                self.index += 1;
                self.run_index = 0;
            } else {
                self.run_index += 1;
            }
        }
        return_value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let values: RunLengthEncoded<i32> = RunLengthEncoded::new();
        assert_eq!(values.run_lengths.len(), 0);
    }

    #[test]
    fn test_store_run() {
        let mut values = RunLengthEncoded::new();
        values.push(7);
        values.push(7);

        assert_eq!(values.run_lengths.len(), 1);
        assert_eq!(values.run_lengths[0].value, 7);
        assert_eq!(values.run_lengths[0].run_length, 2);
    }

    #[test]
    fn test_store_complex() {
        let mut values = RunLengthEncoded::new();
        values.push(1);
        values.push(1);
        values.push(2);
        values.push(7);
        values.push(7);

        assert_eq!(values.run_lengths.len(), 3);
        assert_eq!(values.run_lengths[0].run_length, 2);
        assert_eq!(values.run_lengths[1].run_length, 1);
        assert_eq!(values.run_lengths[2].run_length, 2);
        assert_eq!(values.run_lengths[0].value, 1);
        assert_eq!(values.run_lengths[1].value, 2);
        assert_eq!(values.run_lengths[2].value, 7);
    }

    #[test]
    fn test_iterate_empty() {
        let values = RunLengthEncoded::new();
        let iter_values : Vec<&i32> = values.into_iter().collect();

        assert_eq!(iter_values.len(), 0);
    }

    #[test]
    fn test_iterate_single_item() {
        let mut values = RunLengthEncoded::new();
        values.push(7);
        let iter_values : Vec<&i32> = values.into_iter().collect();

        assert_eq!(iter_values.len(), 1);
        assert_eq!(iter_values[0], &7);
    }

    #[test]
    fn test_iterate_run() {
        let mut values = RunLengthEncoded::new();
        values.push(7);
        values.push(7);
        let iter_values : Vec<&i32> = values.into_iter().collect();

        assert_eq!(iter_values.len(), 2);
        assert_eq!(iter_values[0], &7);
        assert_eq!(iter_values[1], &7);
    }

    #[test]
    fn test_iterate_complex() {
        let mut values = RunLengthEncoded::new();
        values.push(1);
        values.push(1);
        values.push(2);
        values.push(7);
        values.push(7);
        let iter_values : Vec<&i32> = values.into_iter().collect();

        assert_eq!(iter_values.len(), 5);
        assert_eq!(iter_values[0], &1);
        assert_eq!(iter_values[1], &1);
        assert_eq!(iter_values[2], &2);
        assert_eq!(iter_values[3], &7);
        assert_eq!(iter_values[4], &7);
    }
}
