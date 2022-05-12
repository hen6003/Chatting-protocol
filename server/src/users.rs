use tokio::net::tcp::OwnedWriteHalf;

struct User {
    name: String,
    writer: OwnedWriteHalf,
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

    pub fn add(&mut self, name: String, writer: OwnedWriteHalf) -> usize {
        let user = Some(User { name, writer });

        if let Some(empty_id) = self.empty_ids.pop() {
            self.users[empty_id] = user;
            empty_id
        } else {
            self.users.push(user);
            self.users.len() - 1
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.users[id] = None;
        self.empty_ids.push(id);
    }

    pub fn get_name(&mut self, id: usize) -> Result<&String, ()> {
        Ok(&self.users[id].as_ref().ok_or(())?.name)
    }

    pub fn set_name(&mut self, id: usize, name: String) -> Result<(), ()> {
        Ok(self.users[id].as_mut().ok_or(())?.name = name)
    }

    pub fn send_to_all(&self, message: &[u8]) {
        for user in self.users.iter() {
            if let Some(user) = user {
                user.writer.try_write(message).unwrap();
            }
        }
    }
}
