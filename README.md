# findrepo

Finds top-level git repositories in a directory.

It does similar thing as the following commands, but faster:

- ```bash
  fd -t d -H '^\.git$' | sed -e 's,/\.git/$,,'
  ```
- ```bash
  find -type d -execdir test -e '{}/.git' ';' -print -prune
  ```

Typical usage with [fzf][]:

```bash
alias src='cd ~/src/$(findrepo --base-dir ~/src | fzf +m -0 -1)'
```

or if you want <kbd>Ctrl-C</kbd> to not change the directory:

```bash
src() {
    local dir
    dir="$(findrepo --base-dir ~/src | fzf +m -0 -1)" && cd ~/src/"$dir"
}
```

[fzf]: https://junegunn.github.io/fzf/
