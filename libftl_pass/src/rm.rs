
// todo pass which removes node based on its id

pub struct RemoveNode {
    id: usize,
}

impl RemoveNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}