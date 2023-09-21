use crate::app::Invite;
use std::sync::{Mutex, MutexGuard};

pub struct InviteManager {
    invites: Mutex<Vec<Invite>>,
}
impl InviteManager {
    pub fn new() -> Self {
        Self {
            invites: Mutex::new(vec![]),
        }
    }

    pub fn add_invite(&self, invite: Invite) {
        let mut invites = self.invites.lock().unwrap();
        invites.push(invite);
    }

    pub fn get_invites(&self) -> MutexGuard<'_, Vec<Invite>> {
        let mut invites = self.invites.lock().unwrap();
        invites
    }

    pub fn remove_invite(&self, invite_id: i32) {
        let mut invites = self.invites.lock().unwrap();
        invites.retain(|invite| invite.invite_id != invite_id);
    }

    pub fn initialize_with_five_invites(&self) {
        let mut invites = self.invites.lock().unwrap();

        let invite1 = Invite {
            invite_id: 1,
            name: "Alice".to_string(),
            age: 25,
            gender: "Female".to_string(),
            is_stupid: false,
            height: "5ft 5in".to_string(),
        };

        let invite2 = Invite {
            invite_id: 2,
            name: "Bob".to_string(),
            age: 30,
            gender: "Male".to_string(),
            is_stupid: false,
            height: "5ft 9in".to_string(),
        };

        let invite3 = Invite {
            invite_id: 3,
            name: "Charlie".to_string(),
            age: 22,
            gender: "Non-Binary".to_string(),
            is_stupid: true,
            height: "5ft 7in".to_string(),
        };

        let invite4 = Invite {
            invite_id: 4,
            name: "David".to_string(),
            age: 28,
            gender: "Male".to_string(),
            is_stupid: false,
            height: "6ft 1in".to_string(),
        };

        let invite5 = Invite {
            invite_id: 5,
            name: "Eve".to_string(),
            age: 35,
            gender: "Female".to_string(),
            is_stupid: false,
            height: "5ft 4in".to_string(),
        };

        invites.push(invite1);
        invites.push(invite2);
        invites.push(invite3);
        invites.push(invite4);
        invites.push(invite5);
    }
}
