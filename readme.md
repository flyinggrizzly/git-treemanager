>[!NOTE]
> This project is still in-progress. Treat this readme as aspirational for now rather than documenting what this thing
> actually does.

# `git-treemanager`

`git-treemanager` is a git extension, in the spirit of the excellent
[git-absorb](https://github.com/tummychow/git-absorb), and taking inspiration from Rails' "convention over
configuration" philosophy, that makes it easier to manage worktrees.

**The basic convention here is that there is 1 worktree per branch**, which allows the subcommand to assume safety when
creating/cd-ing/rm-ing worktrees.


## Subcommands

All commands must be run from a git repository.

- [`create`](https://github.com/flyinggrizzly/git-treemanager?tab=readme-ov-file#git-treemanager-create-branch-name)
- [`cd`](https://github.com/flyinggrizzly/git-treemanager?tab=readme-ov-file#git-treemanager-cd-branch-name)
- [`rm`](https://github.com/flyinggrizzly/git-treemanager?tab=readme-ov-file#git-treemanager-rm-branch-name)

### `git treemanager create <branch-name>`

Creates a new worktree for the branch. Worktrees are created in `~/worktrees/<reponame>/<branch-name>`

#### `create` options

- `-b` creates the branch, based off the current trunk, like `git checkout -b <branch-name>`
- `--dir` or `-d`: specificies an abritrary directory path to create the worktree in. Example: `git treemanager create my-branch --dir /tmp/whatever/hellyea` would create the worktree in `/tmp/whatever/hellyea`
- `--parent-dir` or `-pd`: specifies the parent directory for the worktree, but will still use `branch-name` as the directory name. Example: `git treemanager -b foo --parent-dir /tmp` would create the worktree in `/tmp/branch-name`
- `--cd` auto-cds to the worktree after creating it. Equivalent to running `git treemanager create my-branch && git treemanager cd my-branch`


### `git treemanager cd <branch-name>`

Moves to the worktree for the for `branch-name`. The worktree must exist.

#### `cd` options

- `--repo` when run from a worktree cds back to the repo's "home" location in the filesystem

### `git treemanager rm <branch-name>`

Deletes the worktree for `branch-name`. Errors if the tree/branch has unmerged commits or is dirty. Like `git branch -d my-branch`.

#### `rm` options

- `-D` forces deletion even if there are unmerged commits. Like `git branch -D my-branch`.
