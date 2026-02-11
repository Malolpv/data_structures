struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

struct HashTable<'a> {
    MAX_INDEX: usize,
    table: Vec<Pair<'a>>,
}

impl<'a> HashTable<'a> {
    fn new(max_size: usize) -> Self {
        Self {
            MAX_INDEX: max_size,
            table: Vec::with_capacity(max_size),
        }
    }

    fn hash(&self, key: &str) -> usize {
        let hash = key.len() + self.MAX_INDEX * 2;
        hash % self.MAX_INDEX
    }

    fn insert(&mut self, key: &'a str, value: &'a str) {
        let index = self.hash(key);
        self.table.insert(index, Pair { key, value });
    }

    fn remove(&mut self, key: &str) {
        let index = self.hash(key);
        self.table.remove(index);
    }

    fn print(&self) {
        self.table
            .iter()
            .for_each(|pair| println!("{}: {}", pair.key, pair.value));
    }
}

fn main() {
    let mut ht = HashTable::new(10);

    ht.insert("Malo", "123");
    ht.insert("Claudie", "123");
    ht.insert("Manue", "123");
    ht.insert("Jerome", "123");
    ht.insert("Zoe", "123");

    ht.print();
}
