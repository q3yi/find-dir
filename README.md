# fdir(find directory)

Find all folders in the given root directory that contain the specified folder or file.

For examples:

Find all git projects in `~/repos` folder:

```shell
> fdir ~/repos
```

Find all rust projects in folder:

```shell
> fdir --has "Cargo.toml" ~/repos
```

After finding all the folders, combining them with the `fzf` and `cd` commands allows us to quickly navigate to the desired project or file directory.

```fish
cd $(echo $REPOS | xargs fdir | fzf)
```

Similar effects can also be achieved using the `find` or `fd` tools, but they cannot stop searching their subdirectories once a matching directory is found. For large projects with many subdirectories, this can consume more time, so I decided to optimize the process."
