//Ask user about his/her car
//Create a struct car
//Populate from user input
//save struct inside of user_info.txt
//Read user_info.txt and print the content on the screen

use std::fs::{File};
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    color: String,
}

//Convert structure to string
impl Car {
    fn to_string(&self) -> String {
        format!(
            "Make: {}\nModel: {}\nColor: {}", self.make, self.model, self.color
        )
    }
}

//Function to save Car struct to file
fn save_to_file(car: &Car) {
    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "{}", car.to_string()).unwrap();
}

//Read from file
fn read_from_file(filename: &str) -> String {
    // Open the file
    let mut file = File::open(filename).unwrap(); 
    
    // Hold info
    let mut info = String::new();
    file.read_to_string(&mut info).unwrap();
    info
}

fn main() {
    
    let mut make = String::new();
    let mut model = String::new();
    let mut color = String::new();
    
    //Ask user for info
    print!("Car make: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut make).unwrap();
    print!("Car model: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut model).unwrap();
    print!("Car color: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut color).unwrap();

    //Create user car 
    let user_car = Car {
        make: make.trim().to_string(),
        model: model.trim().to_string(),
        color: color.trim().to_string(),
    };

    //Save user car into file
    save_to_file(&user_car);

    //Move info from file and read
    let file = read_from_file("user_info.txt");
    println!("\n\nOutput\n{}", file);
}