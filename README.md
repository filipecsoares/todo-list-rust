# Rust TODO List App

This is a simple command-line TODO List application created as a personal project to practice the Rust programming language.

## Features

* List all todos
* Add a todo
* Complete a todo
* Delete a todo
* Edit a todo

## Requirements

* Rust (stable version recommended)
* Cargo (recommended)

## Usage

* Clone the repository:

```bash
git clone https://github.com/filipecsoares/todo-list-rust
```

* Navigate to the project directory:

```bash
cd todo-list-rust
```

* To Build the project:

```bash
cargo build
```

* To add a todo:

```bash
cargo run -- add
```

* To complete a todo:

```bash
cargo run -- complete
```

* To delete a todo:

```bash
cargo run -- remove
```

* To edit a todo:

```bash
cargo run -- edit
```

* To list all todos:

```bash
cargo run -- show
```

### Example

Here's an example of how you can use the application:

* Add a todo

```bash
cargo run -- add
```

Then Enter the task name.

* Complete a todo

```bash
cargo run -- complete
```

Then Enter the task name to be marked as completed.
