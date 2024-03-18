# YAT - Yet Another ToDo List

# Commands


## To create a new task

input:
```
[n|new]
[n|new] "buy milk"
[n|new] "buy milk" #groceries
[n|new] buy milk
[n|new] buy milk --tag=groceries
``` bash

output:
```
New task: 
# Todo
## groceries
1 "buy milk"
``` bash

## List current tasks

input:
```
[l|list]
[l|list] todo
[l|list] doing
[l|list] groceries
[l|list] todo yat
``` bash

output:
```
# Todo
## groceries
1 "buy milk"
2 "buy eggs"
## yat
4 "Add support to dates"

# Doing
## yat
3 "Build list command"
``` bash

## Status

input:
```
[st|status]
``` bash

output:
```
Status

Todo:
    groceries: 2
    yat: 1
    Total: 3 tasks
Doing:
    yat: 1
    Total: 1 task
Done: 35 tasks
``` bash

## Edit / Plan / Organize

input:
```
[e | edit]
[e | edit] todo
[e | edit] groceries
``` bash

Editor input:
```
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


``` bash


output:
```
Updating tasks:
Task #1 "buy milk" | new title: "Buy Milk"
Task #2 "buy eggs" | new title: "Buy Eggs"
Task #5 "buy flour" | new status: "Done" 
``` bash



# Future ideas
* Active task, the task that you are working on the moment
* Interruption - a quick task that will be created as Doing and will be "active"
* Task description, timestamps, effort, importance, order 
* Task statuses configurable
* Support multiple tags

