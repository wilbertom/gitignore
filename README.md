Gitignore
===

Gitignore is a CLI tool for quickly adding `.gitignore`s to projects.

We use [Github's repository](https://github.com/github/gitignore) as a source.

Usage
====

```
gitignore Python

```

Or getting a [global](https://github.com/github/gitignore/tree/master/Global) one:

```
gitignore Global/vim

```


In the future, when I know Rust better
====

Downloads the `gitinore`s over to the `~/.gitinore` directory.

```
gitignore update

```

List all the `gitignore`s.

```
gitignore list

```

Add a `gitignore` to the current directory.

```
gitignore python
```


Add to another directory.

```
gitignore python ~/code/project

```
