struct Kube {
    home: String,
    public_key: String,
    private_key: String
}

fn makeUser(name): Kube {
    return Kube {
        home: '/home/' + name,
        public_key: '/home/' + name + '/.ssh/id_ed25519.pub',
        private_key: '/home/' + name + '/.ssh/id_ed25519'
    };
}

pub let main = [
    makeUser('bob'),
    makeUser('alice')
];