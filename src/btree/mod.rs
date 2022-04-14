use std::borrow::Borrow;

mod btree;
mod node;

trait Searchable<T> {
    fn search(&self, item: impl Borrow<T>) -> Result<usize, usize>;
}

impl<T: PartialOrd> Searchable<T> for Vec<T> {
    fn search(&self, item: impl Borrow<T>) -> Result<usize, usize> {
        let mut pos = 0;
        let item = item.borrow();
        for (idx, it) in self.iter().enumerate() {
            if item == it {
                return Ok(idx);
            }
            if item < it {
                pos = std::cmp::min(pos, idx);
            }
            if item > it {
                pos = idx + 1;
            }
        }
        Err(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::Searchable;

    #[test]
    fn search() {
        let vec = vec![1, 2, 5];

        assert_eq!(Err(0), vec.search(0));
        assert_eq!(Err(2), vec.search(3));
        assert_eq!(Err(3), vec.search(6));
        assert_eq!(Ok(0), vec.search(1));
        assert_eq!(Ok(1), vec.search(2));

        let vec = vec![];
        assert_eq!(Err(0), vec.search(5));

        let vec = vec![4];
        assert_eq!(Ok(0), vec.search(4));
        assert_eq!(Err(0), vec.search(2));
        assert_eq!(Err(1), vec.search(6));
    }
}
