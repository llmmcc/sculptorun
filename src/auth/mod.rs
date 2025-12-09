mod auth;
mod types;

pub use auth::*;
pub use types::*;
pub mod offline;

use offline::OfflineAuthProvider;

pub enum AuthProvider {
    Http(HttpAuthProvider),
    Offline(OfflineAuthProvider), // 添加离线认证
}
