# Rustic Boards

![](https://img.shields.io/badge/license-MIT-green)
![](https://img.shields.io/badge/powered%20by-Rust-blue)
![](https://img.shields.io/badge/crates.io-v0.1.1-blue
)

Rustic Boards is a sleek and powerful CLI-based Kanban board application built with Rust, combining performance and usability for seamless task management. 
It simplifies task tracking and collaboration with an intuitive command line interface and robust features.

Rustic Boards emerges from the philosophy that performance should not be sacrificed for usability. 
Built on the robust foundations of Rust, known for its speed, safety, and memory efficiency, Rustic Boards offers a powerful CLI-based Kanban board experience like no other.

**Note: The application is currently supported only on Windows machines. Support for other operating systems is part of future roadmap.**

## Design

1. Clear and intuitive CLI command structure
2. Simple binary format for storing the Kanban board data
3. Organize tasks into status categories (e.g., "To Do", "In Progress", "Blocked", "In Review", "Done") to represent the workflow stages
4. Add notes for tasks and subtasks to track granular details
5. Robust input validation to handle unexpected user inputs gracefully
6. Provide clear and helpful error messages to guide users when mistakes or issues occur

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
| `add notes <Task or SubTask ID>` | To add notes to an existing task or subtask |
| `show notes <Task or SubTask ID>` | To view notes for an existing task or subtask |
| `filter due <Keyword>` | To filter all tasks and subtasks based on deadline <br> (past-deadline, today, tomorrow, after-tomorrow) |
| `filter priority <Keyword>` | To filter all tasks and subtasks based on priority <br> (high, medium, low) |
| `help` | To view all commands for the application |
| `exit` | To exit the application |

## Installation

1. Build from source:
   - Clone the Git repository (main branch)
   - Install rust toolkit (<https://www.rust-lang.org/tools/install>)
   - Execute command: `cargo build --release`
   - Copy the `rustic_boards.exe` file in "target/release" folder to "C:\rustic_boards"
   - Add the path "C:\rustic_boards" to PATH system environment variable (For more reference, visit <https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)>)
   - You should be able to run `rustic_boards` command on Command Prompt or Powershell now.

## Code Repository

Visit <https://github.com/oss-rust-github-io/rustic_boards> for application source code.
