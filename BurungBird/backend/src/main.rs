use ic_cdk::api::trap;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Thread {
    id: u64,
    title: String,
    content: String,
    author: Principal,
    comments: Vec<Comment>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Comment {
    content: String,
    author: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Ad {
    id: u64,
    title: String,
    description: String,
    category: String,
    price: u64,
    seller: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
struct Event {
    id: u64,
    name: String,
    location: String,
    date: String,
    organizer: Principal,
}

struct BurungBird {
    threads: HashMap<u64, Thread>,
    ads: HashMap<u64, Ad>,
    events: HashMap<u64, Event>,
    next_thread_id: u64,
    next_ad_id: u64,
    next_event_id: u64,
}

impl BurungBird {
    fn new() -> Self {
        Self {
            threads: HashMap::new(),
            ads: HashMap::new(),
            events: HashMap::new(),
            next_thread_id: 1,
            next_ad_id: 1,
            next_event_id: 1,
        }
    }

    // Forum functions
    fn create_thread(&mut self, title: String, content: String, author: Principal) -> u64 {
        let thread = Thread {
            id: self.next_thread_id,
            title,
            content,
            author,
            comments: vec![],
        };
        self.threads.insert(self.next_thread_id, thread);
        self.next_thread_id += 1;
        self.next_thread_id - 1
    }

    fn add_comment(&mut self, thread_id: u64, content: String, author: Principal) -> Result<(), String> {
        if let Some(thread) = self.threads.get_mut(&thread_id) {
            thread.comments.push(Comment { content, author });
            Ok(())
        } else {
            Err("Thread tidak ditemukan".to_string())
        }
    }

    fn list_threads(&self) -> Vec<Thread> {
        self.threads.values().cloned().collect()
    }

    // Marketplace functions
    fn create_ad(&mut self, title: String, description: String, category: String, price: u64, seller: Principal) -> u64 {
        let ad = Ad {
            id: self.next_ad_id,
            title,
            description,
            category,
            price,
            seller,
        };
        self.ads.insert(self.next_ad_id, ad);
        self.next_ad_id += 1;
        self.next_ad_id - 1
    }

    fn list_ads(&self, category: Option<String>) -> Vec<Ad> {
        self.ads
            .values()
            .filter(|ad| category.as_ref().map_or(true, |c| &ad.category == c))
            .cloned()
            .collect()
    }

    // Event functions
    fn create_event(&mut self, name: String, location: String, date: String, organizer: Principal) -> u64 {
        let event = Event {
            id: self.next_event_id,
            name,
            location,
            date,
            organizer,
        };
        self.events.insert(self.next_event_id, event);
        self.next_event_id += 1;
        self.next_event_id - 1
    }

    fn list_events(&self) -> Vec<Event> {
        self.events.values().cloned().collect()
    }
}

static mut BURUNGBIRD: Option<BurungBird> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        BURUNGBIRD = Some(BurungBird::new());
    }
}

#[ic_cdk::update]
fn create_thread(title: String, content: String) -> u64 {
    let caller = ic_cdk::caller();
    let app = unsafe { BURUNGBIRD.as_mut().unwrap() };
    app.create_thread(title, content, caller)
}

#[ic_cdk::query]
fn list_threads() -> Vec<Thread> {
    let app = unsafe { BURUNGBIRD.as_ref().unwrap() };
    app.list_threads()
}

#[ic_cdk::update]
fn create_ad(title: String, description: String, category: String, price: u64) -> u64 {
    let caller = ic_cdk::caller();
    let app = unsafe { BURUNGBIRD.as_mut().unwrap() };
    app.create_ad(title, description, category, price, caller)
}

#[ic_cdk::query]
fn list_ads(category: Option<String>) -> Vec<Ad> {
    let app = unsafe { BURUNGBIRD.as_ref().unwrap() };
    app.list_ads(category)
}

#[ic_cdk::update]
fn create_event(name: String, location: String, date: String) -> u64 {
    let caller = ic_cdk::caller();
    let app = unsafe { BURUNGBIRD.as_mut().unwrap() };
    app.create_event(name, location, date, caller)
}

#[ic_cdk::query]
fn list_events() -> Vec<Event> {
    let app = unsafe { BURUNGBIRD.as_ref().unwrap() };
    app.list_events()
}
