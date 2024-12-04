mod book;
use book::Book;
mod library;
use library::Library;
fn main() {
    //
    let book1 = Book::build("1984", "George Orwell", 1949);
    let book2 = Book::build("Brave New World", "Aldous Huxley", 1932);
    let book3 = Book::build("The Great Gatsby", "F. Scott Fitzgerald", 1925);

    let mut library = Library::build("Exquisite Lib");
    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // library.list_books();
    library.remove_book("1984");
}
