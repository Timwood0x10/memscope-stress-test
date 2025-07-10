//! Complex data structures module for testing memory tracking

use std::collections::{HashMap, VecDeque, BTreeMap, HashSet};
use serde::{Deserialize, Serialize};
// use memtrack_rs::track_var; // Not needed since we're not tracking struct fields

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub posts: Vec<Post>,
    pub followers: HashSet<u64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub likes: u32,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub user_id: u64,
    pub content: String,
    pub replies: Vec<Comment>,
}

#[derive(Default)]
pub struct DataManager {
    pub users: HashMap<u64, User>,
    pub posts_by_tag: BTreeMap<String, Vec<u64>>,
    pub recent_activity: VecDeque<ActivityEvent>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ActivityEvent {
    UserRegistered(u64),
    PostCreated(u64, u64),
}

impl DataManager {
    pub fn new() -> Self {
        let manager = DataManager {
            users: HashMap::new(),
            posts_by_tag: BTreeMap::new(),
            recent_activity: VecDeque::new(),
        };
        
        // Track the manager's components
        // Note: memtrack-rs only supports tracking simple variables, not struct fields
        
        manager
    }

    pub fn create_user(&mut self, id: u64, name: String, email: String) -> Result<(), String> {
        let user = User {
            id,
            name: name.clone(),
            email,
            posts: Vec::new(),
            followers: HashSet::new(),
            metadata: HashMap::new(),
        };
        
        // Track user creation (only basic types supported by memtrack-rs)
        self.users.insert(id, user);
        self.recent_activity.push_back(ActivityEvent::UserRegistered(id));
        
        // Keep only last 1000 activities
        if self.recent_activity.len() > 1000 {
            self.recent_activity.pop_front();
        }
        
        println!("Created user: {} (ID: {})", name, id);
        Ok(())
    }

    pub fn create_post(&mut self, user_id: u64, post_id: u64, title: String, content: String, tags: Vec<String>) -> Result<(), String> {
        let user = self.users.get_mut(&user_id).ok_or("User not found")?;
        
        let post = Post {
            id: post_id,
            title: title.clone(),
            content,
            tags: tags.clone(),
            likes: 0,
            comments: Vec::new(),
        };
        
        // Track post creation
        user.posts.push(post);
        
        // Index by tags
        for tag in tags {
            self.posts_by_tag.entry(tag).or_insert_with(Vec::new).push(post_id);
        }
        
        self.recent_activity.push_back(ActivityEvent::PostCreated(user_id, post_id));
        
        if self.recent_activity.len() > 1000 {
            self.recent_activity.pop_front();
        }
        
        println!("Created post: {} (ID: {}) for user {}", title, post_id, user_id);
        Ok(())
    }

}