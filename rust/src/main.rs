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
    pub last_name: u32,
    pub purchases: Vec<PurchaseStruct>,
}

fn main() {
    println!("Hello, world!");
    let mut purchase = PurchaseStruct {
        purchase_id: String::from("ord-zzz987"),
        name: String::from("laptop"),
        date: String::from("2022-01-25T02:20:42.832Z"),
        price: 853,
    };

    let mut serializedPurchase = Vec::new();
    purchase.serialize(&mut serializedPurchase);
    println!("Serialized length {}", serializedPurchase.len());
    let raw_deserialized = PurchaseStruct::try_from_slice(&serializedPurchase);
    let mut deserialized = match raw_deserialized {
        Ok(T) => {
            println!("Deserialized successfully");
            T
        }
        Err(E) => {
            panic!("Failed to deserialize");
        }
    };

    println!("Deserialized: {}", deserialized.purchase_id);

    // println!("Original  {}", greeting_account.aword);

    // greeting_account.aword = String::from("another one");
    // let mut result = Vec::new();

    // greeting_account.serialize(&mut result);

    // println!("Origin length {}", result.len());

    // let resp = truncate_vec(&result[..]);

    // let truncatedVec = Vec::from(&result[0..resp]);
    // println!("Final {}", truncatedVec.len());
    // let deserialized = GreetingAccount::try_from_slice(&truncatedVec).unwrap();
    // println!("Deserialized again {}", deserialized.aword);
}
