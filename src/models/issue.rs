pub enum IssueStatus {
    Open,
    InProgress,
    Done
}

pub struct Issue {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
    pub board_id: u32, // board ref
    pub assignee_id: u32, // user ref
}