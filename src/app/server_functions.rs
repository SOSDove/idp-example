use std::sync::{Arc, Mutex, Once};
use leptos::{server, ServerFnError};
use serde_derive::{Deserialize, Serialize};
use crate::app::invite_manager::InviteManager;

static INIT: Once = Once::new();

static mut INVITE_MANAGER: Option<Arc<Mutex<InviteManager>>> = None;

#[server(LogThis, "/api")]
pub async fn log_this(message: String) -> Result<(), ServerFnError> {
    println!("This is a {}", message);

    Ok(())
}

#[server(AcceptInvite, "/api")]
pub async fn accept_invite(invite_id: i32) -> Result<String, ServerFnError> {
    println!("Accepting invite with id of {}", invite_id);
    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };
    manager.lock().unwrap().remove_invite(invite_id);
    Ok("Accepted".to_string())
}

#[server(DeclineInvite, "/api")]
pub async fn decline_invite(invite_id: i32) -> Result<String, ServerFnError> {
    println!("Declining invite with id of {}", invite_id);
    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };
    manager.lock().unwrap().remove_invite(invite_id);
    Ok("Declined".to_string())
}

#[server(GetInvites, "/api")]
pub async fn get_invites() -> Result<Vec<Invite>, ServerFnError> {
    println!("Was told to fetch invites");
    INIT.call_once(|| {
        let manager = InviteManager::new();
        manager.initialize_with_five_invites();
        unsafe {
            INVITE_MANAGER = Some(Arc::new(Mutex::new(manager)));
        }
    });

    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };

    let invites;
    {
        let manager = manager.lock().unwrap();
        invites = manager.get_invites().clone();
    }
    Ok(invites.to_vec())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invite {
    pub(crate) invite_id: i32,
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) gender: String,
    pub(crate) is_stupid: bool,
    pub(crate) height: String,
}
