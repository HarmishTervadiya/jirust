use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum IssueStatus {
    Open,
    InProgress,
    Done,
}

pub struct Issue {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
    pub board_id: u32,    // board ref
    pub assignee_id: u32, // user ref
}

impl FromStr for IssueStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(IssueStatus::Open),
            "inprogress" => Ok(IssueStatus::InProgress),
            "done" => Ok(IssueStatus::Done),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}
