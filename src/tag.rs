use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tagger {
    n: i32,
    pub available_ids: Vec<i32>,
}

impl Tagger {
    pub fn new() -> Tagger {
        Tagger {
            n: 0,
            available_ids: Vec::new(),
        }
    }

    pub fn remove_tag(&mut self, idx: i32) {
        self.available_ids.push(idx);
    }

    pub fn new_tag(&mut self) -> i32 {
        let ans = self.available_ids.pop();
        match ans {
            Some(ans) => ans,
            None => {
                self.n += 1;
                self.n - 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::Tagger;

    #[test]
    fn add_remove() {
        let mut t = Tagger::new();
        for i in 0..5 {
            assert_eq!(i, t.new_tag());
        }
        t.remove_tag(1);
        t.remove_tag(3);
        assert!(t.new_tag() < 5);
        assert!(t.new_tag() < 5);
        assert_eq!(5, t.new_tag());
        t.remove_tag(1);
        assert!(t.new_tag() < 6);
        assert_eq!(6, t.new_tag());
        assert_eq!(t.available_ids.len(), 0);
    }
}
