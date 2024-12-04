#![allow(dead_code, unused_variables)]
use crate::book::Book;
pub struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    pub fn build(name: &str) -> Library {
        Library {
            name: name.to_string(),
            books: Vec::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn list_books(&self) {
        for book in &self.books {
            book.display_book_details();
        }
    }

    pub fn remove_book(&mut self, title: &str) {
        self.books.retain(|book| title != book.title);
        self.list_books();
    }
}
