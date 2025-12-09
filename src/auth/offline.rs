use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OfflineAuthProvider {
    players: HashMap<String, OfflinePlayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OfflinePlayer {
    id: Uuid,
    name: String,
    can_upload: bool,
}

impl OfflineAuthProvider {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    // 生成离线UUID（与 Minecraft 一致）
    fn generate_offline_uuid(username: &str) -> Uuid {
        use md5::{Md5, Digest};
        use uuid::Version;
        
        let mut hasher = Md5::new();
        hasher.update(b"OfflinePlayer:");
        hasher.update(username.as_bytes());
        let result = hasher.finalize();
        
        // 转换为UUID格式
        let mut bytes = *result.as_slice();
        bytes[6] = (bytes[6] & 0x0F) | 0x30; // 版本3
        bytes[8] = (bytes[8] & 0x3F) | 0x80; // 变体
        
        Uuid::from_bytes(bytes)
    }

    pub async fn authenticate(&mut self, username: &str) -> Option<OfflinePlayer> {
        let uuid = Self::generate_offline_uuid(username);
        
        let player = OfflinePlayer {
            id: uuid,
            name: username.to_string(),
            can_upload: true,
        };
        
        self.players.insert(username.to_string(), player.clone());
        Some(player)
    }
}
