# Command-Line Help for `mimir`

This document contains the help content for the `mimir` command-line program.

**Command Overview:**

* [`mimir`↴](#mimir)
* [`mimir subject`↴](#mimir-subject)
* [`mimir subject add`↴](#mimir-subject-add)
* [`mimir subject delete`↴](#mimir-subject-delete)
* [`mimir subject rename`↴](#mimir-subject-rename)
* [`mimir file`↴](#mimir-file)
* [`mimir file add`↴](#mimir-file-add)
* [`mimir file delete`↴](#mimir-file-delete)
* [`mimir file rename`↴](#mimir-file-rename)
* [`mimir task`↴](#mimir-task)
* [`mimir task add`↴](#mimir-task-add)
* [`mimir task delete`↴](#mimir-task-delete)
* [`mimir task rename`↴](#mimir-task-rename)
* [`mimir task done`↴](#mimir-task-done)
* [`mimir task undone`↴](#mimir-task-undone)
* [`mimir open`↴](#mimir-open)
* [`mimir tasks`↴](#mimir-tasks)
* [`mimir files`↴](#mimir-files)
* [`mimir workspace`↴](#mimir-workspace)
* [`mimir config`↴](#mimir-config)

## `mimir`

Study Manager CLI

**Usage:** `mimir [search_name] [COMMAND]`

###### **Subcommands:**

* `subject` — 
* `file` — 
* `task` — 
* `open` — 
* `tasks` — 
* `files` — 
* `workspace` — 
* `config` — 

###### **Arguments:**

* `<search_name>` — Optional positional argument for "open" shortcut



## `mimir subject`

**Usage:** `mimir subject <COMMAND>`

###### **Subcommands:**

* `add` — 
* `delete` — 
* `rename` — 



## `mimir subject add`

**Usage:** `mimir subject add <SUBJECT>`

###### **Arguments:**

* `<SUBJECT>`



## `mimir subject delete`

**Usage:** `mimir subject delete <SUBJECT>`

###### **Arguments:**

* `<SUBJECT>`



## `mimir subject rename`

**Usage:** `mimir subject rename <SUBJECT_NAME> <NEW_NAME>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<NEW_NAME>`



## `mimir file`

**Usage:** `mimir file <COMMAND>`

###### **Subcommands:**

* `add` — 
* `delete` — 
* `rename` — 



## `mimir file add`

**Usage:** `mimir file add <SUBJECT> <PATH>`

###### **Arguments:**

* `<SUBJECT>`
* `<PATH>`



## `mimir file delete`

**Usage:** `mimir file delete <FILE_NAME>`

###### **Arguments:**

* `<FILE_NAME>`



## `mimir file rename`

**Usage:** `mimir file rename <FILE_NAME> <NEW_FILE_NAME>`

###### **Arguments:**

* `<FILE_NAME>`
* `<NEW_FILE_NAME>`



## `mimir task`

**Usage:** `mimir task <COMMAND>`

###### **Subcommands:**

* `add` — 
* `delete` — 
* `rename` — 
* `done` — 
* `undone` — 



## `mimir task add`

**Usage:** `mimir task add <SUBJECT_NAME> <TASK_TITLE>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<TASK_TITLE>`



## `mimir task delete`

**Usage:** `mimir task delete <SUBJECT_NAME> <TASK_TITLE>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<TASK_TITLE>`



## `mimir task rename`

**Usage:** `mimir task rename <SUBJECT_NAME> <OLD_TASK_TITLE> <NEW_TASK_TITLE>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<OLD_TASK_TITLE>`
* `<NEW_TASK_TITLE>`



## `mimir task done`

**Usage:** `mimir task done <SUBJECT_NAME> <TASK_TITLE>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<TASK_TITLE>`



## `mimir task undone`

**Usage:** `mimir task undone <SUBJECT_NAME> <TASK_TITLE>`

###### **Arguments:**

* `<SUBJECT_NAME>`
* `<TASK_TITLE>`



## `mimir open`

**Usage:** `mimir open <TARGET>`

###### **Arguments:**

* `<TARGET>` — Name of the file or subject to open



## `mimir tasks`

**Usage:** `mimir tasks [TARGET]`

###### **Arguments:**

* `<TARGET>`



## `mimir files`

**Usage:** `mimir files [SUBJECT_NAME]`

###### **Arguments:**

* `<SUBJECT_NAME>` — Name of the subject to show files for



## `mimir workspace`

**Usage:** `mimir workspace`



## `mimir config`

**Usage:** `mimir config`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
