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
        println!("Password: {:?} ✅", password);
    } else {
        println!("No password found");
    }

    let security_protocol_password = d.find_new_security_protocol_password(&mut safe);

    if let Some(security_protocol_password) = security_protocol_password {
        println!("Password: {:?} ❌ (not validated by adventof code)", security_protocol_password);
    } else {
        println!("No password found");
    }

}
