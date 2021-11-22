import Arr from 'Arr';

struct Kube {
    name.super: String,
    comment: String,
    tags: Arr<String>,
    opt: Number?,
}

fn kube(name: String, hello: String) -> Kube {
    Kube {
        name.super: name,
        comment: hello
    }
}

export Kube;