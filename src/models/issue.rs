enum IssueStatus {
    Open,
    InProgress,
    Done
}

struct Issue {
    id: i32,
    title: String,
    description: String,
    status: IssueStatus,
    board_id: i32, // board ref
    assignee_id: i32 // user ref
}