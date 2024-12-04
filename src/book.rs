#[derive(Debug)]
pub struct Book {
    pub title: String,
    author: String,
    year: u32,
}

impl Book {
    pub fn build(title: &str, author: &str, year: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }

    pub fn display_book_details(&self) {
        println!(
            "The book '{}' is written by {} and published in the year {}",
            self.title, self.author, self.year
        );
    }
}
