enum IssueStatus {
    Open,
    InProgress,
    Done
}

struct Issue {
    id: u32,
    title: String,
    description: String,
    status: IssueStatus,
    board_id: u32, // board ref
    assignee_id: u32 // user ref
}