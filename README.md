# findrepo

Finds top-level git repositories in a directory.

It does similar thing as the following commands, but faster:

- fd -t d -H '^\.git$' | sed -e 's,/\.git/$,,'
- find -type d -execdir test -e '{}/.git' ';' -print -prune

Typical usage:

```bash
alias src='cd ~/src/$(findrepo --base-dir ~/src | fzf +m -0 -1)'
```
