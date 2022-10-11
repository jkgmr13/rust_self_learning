struct Contact {
    name: String,
    address: String,
    age: u32,
    relationship: String,
}

impl Contact {
    fn nice_print(&self) {
        println!("Name: {}\nAddress: {}\nAge: {}\nRelationship: {}", self.name, self.address, self.age, self.relationship);
    }
    
    // This is my best guess for mutators and getters.
    // Haven't gotten far enough in rust book to know if this is correct.
    fn change_address(self, address: String) -> Contact {
        Contact { address: address, ..self }
    }

    fn change_name(self, name: String) -> Contact {
        Contact { name: name, ..self }
    }

    fn change_age(self, age: u32) -> Contact {
        Contact { age: age, ..self }
    }

    fn change_relationship(self, relationship: String) -> Contact {
        Contact { relationship: relationship, ..self }
    }

    // Accessors use a reference to self since I don't want original
    // reference to drop
    fn get_address(&self) -> String { self.address.clone() }

    fn get_name(&self) -> String { self.name.clone() }

    fn get_age(&self) -> u32 { self.age }

    fn get_relationship(&self) -> String { self.relationship.clone() }
}

fn main() {
    let steven = Contact {
        name: "steven".to_string(),
        address: "123 Japan Way, Japan".to_string(),
        age: 24,
        relationship: "Best Friend".to_string(),
    };

    let brandon = Contact {
        name: "brandon".to_string(),
        address: "456 California dr, California".to_string(),
        age: 18,
        relationship: "Friend".to_string(),
    };

    //Since above were just references original creation should still work.
    steven.nice_print();
    brandon.nice_print();

    let steven = steven.change_address("America now, not Japan".to_string());
    let steven = steven.change_name("Don".to_string());
    let steven = steven.change_age(45);
    let steven = steven.change_relationship("Brother from another mother".to_string());
    steven.nice_print();


    println!("Testing accessors\n
    Age: {}\n
    Address: {}\n
    Name: {}\n
    Relationship: {}",
    brandon.get_age(), brandon.get_address(), brandon.get_name(), brandon.get_relationship());
}

