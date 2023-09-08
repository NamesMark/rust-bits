#[derive(Debug)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn main() {
    let invitation1 = Invitation::new("dolores".to_string(), true, None);
    let invitation2 = Invitation::new("james".to_string(), false, Some("Sorry, cannot".to_string()));

    println!("{invitation1:#?}");
    println!("{invitation2:#?}");
}
