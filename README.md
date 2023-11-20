# git-finder

Find all git projects under given root folder.

Usage:

```shell
> git-finder ~/repos
```

A more useful example, quick jump to project folder:

```shell
cd $(echo $REPOS | xargs git-finder | fzf)
```

We could do the same with `find` or `fd` command, but they can not stop search
sub folder in a git project, so I write one for my self.
