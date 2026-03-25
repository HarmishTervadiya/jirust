// TODO: have to think i32 or u32 for keys (ids)
pub struct AppState {
    pub organizations: HashMap<u32, Organization>, // org
    pub users: HashMap<u32, User>, // user
    pub boards: HashMap<u32, Board>, // board
    pub issues: HashMap<u32, Issue>, // issue 
    pub next_id: u32,   // auto-increment, like MongoDB ObjectId counter

}