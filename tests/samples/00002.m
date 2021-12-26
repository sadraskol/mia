import Array from 'Array';

struct Kube {
    name.super: String,
    comment: String,
    tags: Array<String>,
    opt: Number?,
}

let ratio = 0.34;
let comment = '12312';
let tag_1 = 'tag 1';
let tag_2 = 'tag 2';

pub let main = Kube {
    name.super: 'Some name',
    comment: comment,
    tags: [ tag_1, ratio ],
    opt: nil
};