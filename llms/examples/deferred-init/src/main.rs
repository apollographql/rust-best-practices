use std::cell::OnceCell;

struct Document {
    text: String,
    word_count: OnceCell<usize>,
}

impl Document {
    fn new(text: String) -> Self {
        Self {
            text,
            word_count: OnceCell::new(),
        }
    }

    fn word_count(&self) -> usize {
        *self
            .word_count
            .get_or_init(|| self.text.split_whitespace().count())
    }
}

fn main() {
    let document = Document::new("deferred local computation".to_owned());
    println!("{} words", document.word_count());
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn word_count_is_initialized_on_demand() {
        let document = Document::new("  two\nwords  ".to_owned());
        assert!(document.word_count.get().is_none());
        assert_eq!(document.word_count(), 2);
        assert_eq!(document.word_count.get(), Some(&2));
        assert_eq!(document.word_count(), 2);
    }
}
