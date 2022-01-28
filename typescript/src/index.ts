import * as borsh from 'borsh';

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

const PurchaseSchema = new Map<any, any>([
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
    ]
]);


const purchase = new PurchaseStruct({
    purchase_id: 'ord-zzz987',
    name: 'laptop',
    date: '2022-01-25T02:20:42.832Z',
    price: 853,
});

const serialized = borsh.serialize(PurchaseSchema, purchase);
console.log(serialized.length);
