use crate::document::Document;
use crate::safe::Safe;

mod document;
mod rotation;
mod safe;

fn main() {
    let d = Document::from_file("input.txt");
    let mut safe = Safe::with_arrow(50);

    let password = d.find_password(&mut safe);

    if let Some(password) = password {
        println!("Password: {:?}", password);
    } else {
        println!("No password found");
    }

}
