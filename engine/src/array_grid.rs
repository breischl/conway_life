#[derive(Clone)]
pub struct ArrayGrid<T>
where
    T: Clone + Default,
{
    col_size: usize,
    arr: Vec<T>,
}

impl<T> ArrayGrid<T>
where
    T: Clone + Default,
{
    pub fn set(&mut self, xu: usize, yu: usize, val: T) {
        let idx = self.get_index(xu, yu);
        self.arr[idx] = val;
    }

    pub fn get(&self, xu: usize, yu: usize) -> &T {
        let idx = self.get_index(xu, yu);
        &self.arr[idx]
    }

    pub fn get_mut(&mut self, xu: usize, yu: usize) -> &mut T {
        let idx = self.get_index(xu, yu);
        &mut self.arr[idx]
    }

    fn get_index(&self, xu: usize, yu: usize) -> usize {
        yu + xu * self.col_size
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.arr.iter()
    }

    pub fn create(row_size: usize, col_size: usize) -> ArrayGrid<T> {
        let size = row_size * col_size;
        let mut arr: Vec<T> = Vec::with_capacity(size);
        arr.resize(size, Default::default());

        ArrayGrid { col_size, arr }
    }
}
