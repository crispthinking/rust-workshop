//! Book Module
//!
//! This module contains the definition for the Book type and
//! associated functionality.

/// A Library Book
#[derive(Debug)]
pub struct Book {
    /// The title of the book
    title: String,

    /// The page this book is open at
    page: i32,
}


impl Book {

    /// Create a New Book
    pub fn new(title: &str) -> Self {
        Book {
            title: title.into(),
            page: 1,
        }
    }
}
