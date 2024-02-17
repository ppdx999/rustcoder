# rustcoder

Personal repository for atcoder in rust lang


# setup

install [cargo-compete](https://github.com/qryxip/cargo-compete)

# Start new contest

```sh
$ cargo compete n {{contest_name}}
$ cd {{contest_name}}
$ nvim
$ cargo compete s {{problem_name}}
```

example for atcoder abc335_c

```sh
$ cargo compete n abc335
$ cd abc335
$ nvim # write code for 'c' problem
$ cargo compete s c
```


# Q and A

## Q. Submit fail with 'Submission rejected'

A. Maybe wrong `language_id` in compete.toml. The latest `language_id`is `5054` in 2024/01/08.
