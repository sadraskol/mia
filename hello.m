struct Kube {
    home: String,
    public_key: String,
    private_key: String
}

let users = ['bob', 'alice'];

fn makeUser(name: String): Kube {
    return Kube {
        home: '/home/' + name,
        public_key: '/home/' + name + '/.ssh/id_ed25519.pub',
        private_key: '/home/' + name + '/.ssh/id_ed25519'
    };
}

pub let main = map user in users {
    return makeUser(user);
};