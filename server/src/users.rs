use tokio::net::tcp::OwnedWriteHalf;

pub struct User {
    name: String,
    writer: OwnedWriteHalf,
}

impl User {
    pub fn new(name: String, writer: OwnedWriteHalf) -> Self {
        Self { name, writer }
    }

    pub fn get_name(&mut self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}

pub struct Users {
    users: Vec<Option<User>>,
    empty_ids: Vec<usize>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            empty_ids: Vec::new(),
        }
    }

    pub fn add(&mut self, user: User) -> usize {
        if let Some(empty_id) = self.empty_ids.pop() {
            self.users[empty_id] = Some(user);
            empty_id
        } else {
            self.users.push(Some(user));
            self.users.len() - 1
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.users[id] = None;
        self.empty_ids.push(id);
    }

    pub fn get(&mut self, id: usize) -> &mut User {
        self.users[id].as_mut().unwrap()
    }

    pub fn send_to_all(&self, message: &[u8]) {
        for user in self.users.iter() {
            if let Some(user) = user {
                user.writer.try_write(message).unwrap();
            }
        }
    }
}
