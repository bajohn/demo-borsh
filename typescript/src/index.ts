import * as borsh from 'borsh';
import { deserialize } from 'v8';

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
console.log('\nLength of serialized byte array:')
console.log(serialized.length);

const deserialized = borsh.deserialize(
    PersonSchema,
    PersonStruct,
    Buffer.from(serialized)
);
console.log('\nPrint after deserialization:');
console.log(JSON.stringify(deserialized, null, 2));