# Mimir

**Mimir** is a lightweight study manager that helps you organize and access your files quickly.  
Designed for students and anyone who wants to keep study materials at hand without hassle.

## Features

- Quickly add and store files
- Easy access and search for your materials

## Status

⚠️ **Work in progress** — Mimir is currently under active development.

## Autocompletion
mimir provides zsh autocompletion with dynamic suggestions for:

`mimir open <name>`

where <name> is a subject or file name.

### Installation (user-local)

```
mkdir -p ~/.zsh/completions
mimir completions zsh > ~/.zsh/completions/_mimir
```
Add the following to your ~/.zshrc (if not already present):
```
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit
compinit
```
Restart your shell or reload the config:

`source ~/.zshrc`


**Command Overview:**

* `mimir`
* `mimir subject`
* `mimir subject add`
* `mimir subject delete`
* `mimir subject rename`
* `mimir file`
* `mimir file add`
* `mimir file delete`
* `mimir file rename`
* `mimir task`
* `mimir task add`
* `mimir task delete`
* `mimir task rename`
* `mimir task done`
* `mimir task undone`
* `mimir open`
* `mimir tasks`
* `mimir files`
* `mimir workspace`
* `mimir config`
