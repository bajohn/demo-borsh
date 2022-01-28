# Intro
The Borsh (link to https://borsh.io/) library provides a standard to bijectively serialize and deserialize arbitrary data structures. There are a number of trivial examples available online demonstrating how to serialize flat data structures Borsh.  In scaling up a data structure, we can often use the principle of mathematical induction, whereby one only needs to prove a base case of (n=0) and an induction step (n+1) to scale from 0 to infinity. Most 

The purpose of this article is to demonstrate the induction case for Borsh serialization schemata in Typescript and Rust, meaning the ability to nest data structures. This allows a developer to create schemata to ship arbitrarily complex data structures between the two languages. 

# Sample Schema
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
            "date": "2022-01-25T02:20:42.832Z"
        },
    ]
    
}
```
This data structure would be trivial to represent with a standard Typescript interface, however, Borsh requires a specific Schema and Class arrangement to define the data structure. The above can be represented by: 
```
interface 
```