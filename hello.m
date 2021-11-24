import Arr from 'Arr';

pub struct Kube {
    name.super: String,
    comment: String,
    tags: Arr<String>,
    opt: Number?,
}

let ratio = 0.34;

pub let init = Kube {
    name.super: 'Some name',
    comment: 'Comment',
    tags: [],
    opt: 123 * ratio
};