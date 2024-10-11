use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
        // Create the file to write the books
        let mut file = File::create(filename).unwrap();

        // Write into file
        for book in books {
            writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap();
        }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut books = Vec::new();

    //Read lines from the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(',').collect();
        
        let title = parts[0].to_string();
        let author = parts[1].to_string();
        let year = parts[2].parse::<u16>().unwrap();

        books.push(Book { title, author, year });
    }

    return books;
    
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "Holes".to_string(), author: "Louis Sachar".to_string(), year: 1998 },

    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    } 
}