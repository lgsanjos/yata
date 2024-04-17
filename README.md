# YAT - Yet Another ToDo List

# Commands

## Edit / Organize / New tasks

input:
``` bash
[e | edit]
```

Editor input:
``` bash
#### Edit tasks
#### Change task titles, remove lines to delete task or move lines to update their status/category.

### Todo

## groceries
1 "Buy Milk"
2 "Buy eggs"

## yat
4 "Add support to dates"

### Doing

## yat
3 "Build list command"

### Done - Move tasks here to mark as done
5 "buy flour"

```


## List current tasks

input:
``` bash
[l|list]
[l|list] --done
```

output:
``` bash
# Todo
## groceries
1 "buy milk"
2 "buy eggs"
## yat
4 "Add support to dates"

# Doing
## yat
3 "Build list command"
```

## Status

input:
``` bash
[st|status]
```

output:
``` bash
Status

Todo:
    groceries: 2
    yat: 1
    Total: 3 tasks
Doing:
    yat: 1
    Total: 1 task
Done: 35 tasks
```

output:
``` bash
Updating tasks:
Task #1 "buy milk" | new title: "Buy Milk"
Task #2 "buy eggs" | new title: "Buy Eggs"
Task #5 "buy flour" | new status: "Done" 
```

# Future ideas
* Active task, the task that you are working on the moment
* Take over task - a quick task that will be created as Doing and will be "active"
* Task description, timestamps, effort, weight 
* Task statuses and projects pre-configurable with order
* Report: Show tasks marked as done yesterday
* Report: Show what was done this week (grouped by day)
* Scheduled tasks: New syntax to set scheduled tasks deadlines
* Recurring tasks: Similar to scheduled tasks but this keeps creating or showing the same task in a given frequency
* Display active task title in $PS2 (configurable and consider support with spaceship first)

