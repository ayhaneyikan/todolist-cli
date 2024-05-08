# Todo List CLI

This is a little personal project addressing my desire for a simple command line todo list.
It is quite possible that such a program already exists with more capability, however, I thought this would be a fun little project to explore nonetheless.
Feel free to try it out by cloning the source and building it locally, or download a binary from a release!

Primarily the focus of this project is to explore two things:
1. the Rust programming language
2. creating and deploying my own executable program on a linux system

## Retrieve Binary

### Linux Release

At the time of writing a release is available directly for Linux systems.
See the Github `Releases` section to retrieve this if on a Linux system.

### Building From Source

Otherwise, install `rustup` in order to be able to build the project.
See the appropriate steps for your system within [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Clone the source from this repository.
```bash
git clone git@github.com:ayhaneyikan/todolist-cli.git
```

Build the project in release mode (includes optimizations).
```bash
cargo build --release
```
The created release will be available within the project at `target/release/todo`.
Either add this binary to your PATH or move the binary to an existing PATH location.
Search online for best practices based on your operating system.

## Usage

Once available on your command line, using the `todo` tool is straightforward.
Any questions should be answerable using `-h` or `--help` on the command or any subcommands.
e.g.,
```bash
todo -h
todo tasks -h
```

### Basic Functionality

The following section will highlight some key functionality of the `todo` tool using an example project.

#### Creating a New List

Lets create an example todo list for a school project.
```bash
# todo create <list-name>
todo create my-website
```
Our mock project is to create a website.
Note that list names must start with an english letter or a number and may only contain more of the same along with hyphens or underscores.
`todo` will remind you of this if you attempt to create a list with an invalid name.

#### Viewing, Focusing, and Deleting Lists

With `todo` you can maintain several lists at once.
However, there is only ever one list at a time which is *focused*.
This functions similarly to how `git` allows you to edit a single branch at a time.
We'll dive into the details of this in the following section.

Here's how you can view and switch between lists.
There is also a shortcut for convenience...
```bash
# ALIASES:
# todo list
# todo ls

todo ls
# displays available lists e.g.,
# * my-website *

# we can create another list just to see it in this list
todo create another-list
todo ls
# displays available lists
#   another-list
# * my-website *


# we can then switch to this new list
todo focus another-list
# this allows you to update the tasks within this list (see next section)
# notice how the todo ls output changes to reflect your focus
todo ls
# * another-list *
#   my-website


# remove this unnecessary list
todo delete another-list
# re-type the list name to confirm the delete
# notice that after delete, todo automatically focused our other list
todo ls
#   another-list
# * my-website *
```

#### Viewing, Adding, and Removing Tasks

Naturally we want to track items to do.
These are called `Tasks` and we can easily add these to our list.
Like `todo ls` above, this command has a shorter alias for convenience...
```bash
# ALIASES
# todo tasks
# todo ts

todo ts
# displays the list name and tasks in the list below that e.g.,
# -- my-website --

# clearly we have no tasks in our list
# add task(s) to the list
# we can add multiple at once, but note that they're separated by spaces
todo add task1 task2 task3
todo ts
# -- my-website --
# 1| ✕ task1
# 2| ✕ task2
# 3| ✕ task3


# to add a task with a more descriptive name, use bash quotes (single or double)
# note that this lets you incorporate environment variables into tasks if you want
todo add 'Longer description of what we need'
todo add 'This time I want to add two tasks' "Here's the second one"
todo ts
# -- my-website --
# 1| ✕ task1
# 2| ✕ task2
# 3| ✕ task3
# 4| ✕ Longer description of what we need
# 5| ✕ This time I want to add two tasks
# 6| ✕ Here's the second one


# lets remove some of these silly tasks
# tasks are removed based on their index for simplicity
todo drop 6
todo drop 4 5  # note that indices may change after a previous removal
todo ts
# -- my-website --
# 1| ✕ task1
# 2| ✕ task2
# 3| ✕ task3
```

#### Completing Tasks

Mark one or more tasks as completed using...
```bash
# ALIASES
# todo done <task-index> ...
# todo do <task-index> ...

todo do 1 2
todo ts
# -- my-website --
# 1| ✓ task1
# 2| ✓ task2
# 3| ✕ task3
```

Oops! If you accidentally marked a task you shouldn't have, undo it...
```bash
# todo undo <task-index> ...
todo undo 2
todo ts
# -- my-website --
# 1| ✓ task1
# 2| ✕ task2
# 3| ✕ task3
```

### Advanced Functionality

#### Viewing All Tasks

Sometimes you may want to view tasks across all your lists (not just the one focused).
To do this, use the `-a` option...
```bash
todo tasks -a
todo ts -a
```
It will output all lists with the tasks for that list directly below the list name.

#### Adding Tasks with Dates

When adding a task (or group of tasks) you can attach a date to those tasks.
Keep in mind this means attaching a *single* date to *all* tasks added in that command.
Do this using the `-d` option.
This option parses common date formats and supports hyphens or forward slashes.
It will give you feeback if you provide an invalid date.
```bash
todo add some more tasks -d 4-30
todo ts
# -- my-website --
# 1| ✓ task1
# 2| ✕ task2
# 3| ✕ task3
# 4| ✕ [04/30] some
# 5| ✕ [04/30] more
# 6| ✕ [04/30] tasks

# tasks are sorted by date
# lets add a few more dates to see this in action
todo add before --date 02/14
todo add after -d 06/25
# -- my-website --
# 1| ✓ task1
# 2| ✕ task2
# 3| ✕ task3
# 4| ✕ [02/14] before
# 5| ✕ [04/30] some
# 6| ✕ [04/30] more
# 7| ✕ [04/30] tasks
# 8| ✕ [06/25] after

# years are also supported
# any date without a year specified will be assumed to be within the current year
todo add old-year -d 03/17/2001
todo add curr-year -d 03/17/2024
# -- my-website --
# 1 | ✓ task1
# 2 | ✕ task2
# 3 | ✕ task3
# 4 | ✕ [03/17/2001] old-year
# 5 | ✕ [02/14] before
# 6 | ✕ [03/17/2024] curr-year
# 7 | ✕ [04/30] some
# 8 | ✕ [04/30] more
# 9 | ✕ [04/30] tasks
# 10| ✕ [06/25] after
```

---
Enjoy!
