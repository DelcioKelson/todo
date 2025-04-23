# todo
Todo manager in rust

# Features
- Add tasks
- List tasks
- Mark tasks as done
- Remove tasks
- Search tasks
- Sort tasks
- Filter tasks
- Edit tasks
- Undo tasks
- Redo tasks
- Help
- Exit
- Version

# Usage
```bash
cargo run
```
# Commands
```bash
add <task>         # Add a task
list               # List all tasks
done <task_id>     # Mark a task as done
remove <task_id>   # Remove a task
search <query>     # Search tasks
sort <field>      # Sort tasks by field
filter <field>    # Filter tasks by field
edit <task_id>     # Edit a task
help               # Show help
exit               # Exit the program
version            # Show version
```
# Example
```bash
add "Buy milk"
add "Buy bread"
list --search "milk" --sort "name" --filter "done"
remove 2
edit 1 "Buy eggs"
undo
redo
help
exit
version
```
# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
