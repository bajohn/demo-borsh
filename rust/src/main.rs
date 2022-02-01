use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PurchaseStruct {
    pub purchase_id: String,
    pub name: String,
    pub price: u32,
    pub date: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PersonStruct {
    pub person_id: String,
    pub first_name: String,
    pub last_name: String,
    pub purchases: Vec<PurchaseStruct>,
}

fn main() {
    let laptop_purchase = PurchaseStruct {
        purchase_id: String::from("ord-zzz987"),
        name: String::from("laptop"),
        date: String::from("2022-01-25T02:20:42.832Z"),
        price: 853,
    };

    let headphones_person = PurchaseStruct {
        purchase_id: String::from("ord-yyy654"),
        name: String::from("headphones"),
        date: String::from("2022-01-23T14:12:05.631Z"),
        price: 63,
    };

    let person = PersonStruct {
        person_id: String::from("usr-abc123"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        purchases: Vec::from([laptop_purchase, headphones_person]),
    };
    let to_print = fmt_person(&person);
    println!("Print before serialization: {}", to_print);
    let mut serialized_person = Vec::new();
    let serialized_result = person.serialize(&mut serialized_person);
    match serialized_result {
        Ok(r) => {
            println!("Serialized successfully.");
            r
        }
        Err(_e) => {
            panic!("Failed to serialize!");
        }
    };
    println!(
        "Length of serialized byte array: {}.",
        serialized_person.len()
    );
    let raw_deserialized = PersonStruct::try_from_slice(&serialized_person);
    let deserialized = match raw_deserialized {
        Ok(r) => {
            println!("Deserialized successfully.");
            r
        }
        Err(_e) => {
            panic!("Failed to deserialize!");
        }
    };

    println!("Print after deserialization: {}", fmt_person(&deserialized));
}

// Formatter helper functions
fn fmt_purchase(purchase: &PurchaseStruct) -> String {
    format!(
        "
        ID: {}
        Name: {}
        Date: {}
        Price: {}
    ",
        purchase.purchase_id, purchase.name, purchase.date, purchase.price
    )
}

fn fmt_person(person: &PersonStruct) -> String {
    let mut purchases_formatted = String::from("");
    for purchase in &person.purchases {
        let purchase_formatted = fmt_purchase(&purchase);
        purchases_formatted.push_str(&purchase_formatted);
    }
    format!(
        "
    ID: {}
    Name: {} {}
    Purchases: {}
    ",
        person.person_id, person.first_name, person.last_name, purchases_formatted
    )
}
