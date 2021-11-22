import Arr from 'Arr';

struct Kube {
    name.super: String,
    comment: String,
    tags: Arr<String>,
    opt: Number?,
}

export Kube;

let ratio = 0.34;

export init = Kube {
    name.super: 'Some name',
    comment: 'Comment',
    tags: [],
    opt: 123 * ratio
};