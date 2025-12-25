use rand::{Rng, rng};
use rand_distr::Alphanumeric;

impl Playlist {
    //** Getters **//
    pub fn get_id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    pub fn get_name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    pub fn get_source(&self, index: usize) -> Option<String> {
        self.sources.get(index).cloned()
    }

    pub fn get_sources(&self) -> Option<Vec<String>> {
        Some(self.sources.clone())
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    //** Misc. **//
    pub fn add_source(&mut self, source: Option<String>) {
        self.sources.push(source.unwrap());
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn new(name: String) -> Self {
        Self {
            // Arrays
            sources: Vec::new(),

            // Strings
            id: (0..16).map(|_| rng().sample(Alphanumeric) as char).collect(),
            name
        }
    }

    pub fn remove_source(&mut self, source: String) {
        let mut index = 0;

        for entry in self.sources.clone() {
            if source.clone() == entry { self.sources.remove(index); }
            else { index += 1; };
        };
    }

    //** Setters **//
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Clone, Debug)]
pub struct Playlist {
    // Arrays
    sources: Vec<String>,

    // Strings
    id: String,
    name: String
}