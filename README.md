# Rustic Boards

![](https://img.shields.io/badge/license-MIT-green)
![](https://img.shields.io/badge/powered%20by-Rust-blue)
![](https://img.shields.io/badge/crates.io-v0.1.0-blue
)

Rustic Boards is a sleek and powerful CLI-based Kanban board application built with Rust, combining performance and usability for seamless task management. 
It simplifies task tracking and collaboration with an intuitive command line interface and robust features.

Rustic Boards emerges from the philosophy that performance should not be sacrificed for usability. 
Built on the robust foundations of Rust, known for its speed, safety, and memory efficiency, Rustic Boards offers a powerful CLI-based Kanban board experience like no other.

## Design

1. Clear and intuitive CLI command structure
2. Simple binary format for storing the Kanban board data
3. Organize tasks into status categories (e.g., "To Do", "In Progress", "Blocked", "In Review", "Done") to represent the workflow stages
4. Robust input validation to handle unexpected user inputs gracefully
5. Provide clear and helpful error messages to guide users when mistakes or issues occur

## CLI Commands

| Command | Description |
| ------- | ----------- |
| `add task` | To add a new task into board (along with subtasks - optional) |
| `add subtask` | To add a new subtask into board and link to a parent task |
| `edit task <Task ID>` | To modify details for a task or to create new subtasks under a task <br> (Note: only task description, priority, deadline can be modified) |
| `edit subtask <SubTask ID>` | To modify details for a subtask <br> (Note: only subtask description, priority, deadline and linked parent task can be modified) |
| `open task <Task ID>` | To view all details for a task |
| `open subtask <SubTask ID>` | To view all details for a subtask |
| `delete task <Task ID>` | To delete a task (This will delete all related subtasks too) |
| `delete subtask <SubTask ID>` | To delete a subtask (This won't have any impact on the parent task) |
| `move task <Task ID> <Swimlane>` | To move a task across different swimlanes on board |
| `move subtask <SubTask ID> <Swimlane>` | To move a subtask across different swimlanes on board |
| `link subtask <SubTask ID>` | To link a subtask to different parent task |
| `show task <Swimlane>` | To view all tasks in given swimlane <br> (to-do, in-progress, blocked, in-review, done, all) |
| `show subtask <Swimlane>` | To view all subtasks in given swimlane <br> (to-do, in-progress, blocked, in-review, done, all) |
| `filter due <Keyword>` | To filter all tasks and subtasks based on deadline <br> (past-deadline, today, tomorrow, after-tomorrow) |
| `filter priority <Keyword>` | To filter all tasks and subtasks based on priority <br> (high, medium, low) |
| `help` | To view all commands for the application |
| `exit` | To exit the application |
