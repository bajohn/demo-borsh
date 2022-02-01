# Intro
In scaling up a data structure, we can often use the principle of mathematical induction, where we prove a base case of (n=0) and an induction step (n+1) to scale from zero to infinity. 

The Borsh (link to https://borsh.io/) library provides a standard to bijectively serialize and deserialize data structures. There are a number of examples available online demonstrating the base case: serializing a flat Borsh schema, like a simple dict or array.

The purpose of this article is to use Typescript and Rust to demonstrate the induction step, meaning serializing and deserializing nested data structures. This allows a developer to use Borsh to ship arbitrarily complex data structures between the two languages.

The full, runnable source code can be found here (link to https://github.com/bajohn/borsh-demo).

# Sample Data Model
Consider the JSON object
```
{
    "person_id": "usr-abc123"
    "first_name": "John",
    "last_name": "Doe",
    "purchases": [
        {
            "purchase_id": "ord-zzz987",
            "name": "laptop",
            "price": 853,
            "date": "2022-01-25T02:20:42.832Z"
        },
        {
            "purchase_id": "ord-yyy654",
            "name": "headphones",
            "price": 63,
            "date": "2022-01-23T14:12:05.631Z"
        },
    ]
    
}
```
# Typescript Example 
This data model can be represented by the following typescript interfaces:
```
interface iPurchase {
    purchase_id: string;
    name: string;
    price: number;
    date: string;
};

interface iPerson {
    person_id: string;
    first_name: string;
    last_name: string;
    purchases: iPurchase[];
};
 
```
Borsh requires the data structure to be represented using Typescript classes as well as schema definitions using the Javascript `Map` object:
```
class PurchaseStruct implements iPurchase {
    purchase_id: string;
    name: string;
    price: number;
    date: string;
    constructor(fields: iPurchase) {
        Object.assign(this, fields);
    };

};


class PersonStruct implements iPerson {
    person_id: string;
    first_name: string;
    last_name: string;
    purchases: iPurchase[];
    constructor(fields: iPerson) {
        Object.assign(this, fields);
    }
};

const PersonSchema = new Map<any, any>([
    [
        PersonStruct, {
            kind: 'struct',
            fields: [
                ['person_id', 'string'],
                ['first_name', 'string'],
                ['last_name', 'string'],
                ['purchases', [PurchaseStruct]],
            ]
        }
    ],
    [
        PurchaseStruct, {
            kind: 'struct',
            fields: [
                ['purchase_id', 'string'],
                ['name', 'string'],
                ['price', 'u32'],
                ['date', 'string'],
            ]
        }
    ]
]);
```

With these definitions, we can create some sample data and serialize/deserialize using Borsh:
```
const laptopPurchase = new PurchaseStruct({
    purchase_id: 'ord-zzz987',
    name: 'laptop',
    date: '2022-01-25T02:20:42.832Z',
    price: 853,
});

const headphonesPurchase = new PurchaseStruct({
    purchase_id: 'ord-yyy654',
    name: 'headphones',
    date: '2022-01-23T14:12:05.631Z',
    price: 63,
});

const person = new PersonStruct({
    person_id: 'usr-abc123',
    first_name: 'John',
    last_name: 'Doe',
    purchases: [
        laptopPurchase,
        headphonesPurchase
    ]
});

console.log('\n\nPrint before serialization:');
console.log(JSON.stringify(person, null, 2));


const serialized = borsh.serialize(PersonSchema, person);
console.log('\nSerialized successfully.');
console.log(`Length of serialized byte array: ${serialized.length}.`);
console.log('Deserialized successfully.');

const deserialized = borsh.deserialize(
    PersonSchema,
    PersonStruct,
    Buffer.from(serialized)
);
console.log('\nPrint after deserialization:');
console.log(JSON.stringify(deserialized, null, 2));
```
# Rust Example 
In Rust, structs decorated with `derive` macros provided by the Borsh library can be used to represent the data model. These macros extend the structs with methods required for Borsh serialization and deserialization.

```
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
```
With these definitions, we can create the same sample data and serialize/deserialize using Borsh:

```
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
```

# Conclusion

