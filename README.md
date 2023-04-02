# soyuz-cli

This is the sister application to [`soyuz-web`](https://github.com/hughrun/soyuz-web).

`soyuz-cli` can be used alone or on combination with `soyuz-web` to publish and syncronise a gemlog between your local machine and your gemini server.

It is mostly a wrapper around `rsync`. Additionally it will maintain an archive of posts for each year, and the latest 5 posts on your gemlog homepage.

## Assumptions

1. You are using MacOS locally and a unix-like OS on the server (it probably works on n*x locally but I haven't tested it)
2. If you have text _under_ the list of latest posts on your homepage, you have at least 5 posts listed already (I could have put in some logic to deal with this but I am lazy)
3. You have `rsync` and `ssh` installed on your local machine
4. You have read and write permission for your gemlog files on the server, with the same username as your local machine (otherwise file permissions get weird)
5. You have permission to install files at `/usr/local/bin` on your local machine
6. You publish posts one at a time, so each post will be published prior to the next one being drafted

## Installation

The easiest way to install `soyuz-cli` is using the install script. If you are using MacOS, run this command in `Terminal`:

```sh
curl -L https://hugh.run/install-soyuz | bash
```

Alternatively, you can build from source if you have rust and cargo installed.

## Set up

Once installed, run `soyuz settings` to create a new settings file. The settings file has the following values:

* `local_dir` - the directory on your local machine where your Gemini files are saved
* `remote_dir` - the SSH remote directory path to your files on the server. This includes the server name or ip address, so should look something like `gemini-server:/srv/gemini/example.com` or `username@123.456.789:/srv/gemini/example.com`.
* `editor` - the command to open the text editor you want to use. Defaults to `nano`.
* `index_heading` - the heading text above the latest posts listing on your homepage. If unsure, leave this on the default.

## Commands

Run `soyuz help` for a list of commands and what they do.