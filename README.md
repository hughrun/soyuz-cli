# soyuz-cli

This is the sister application to `soyuz-web`.

`soyuz-cli` can be used alone or on combination with `soyuz-web` to publish and syncronise a gemlog between your local machine and your gemini server.

It is essentially a wrapper around `rsync`. Additionally it will maintain an archive of posts for each year, and the latest 5 posts on your gemlog homepage.

## Assumptions

1. You are using MacOS locally and a unix-like OS on the server (it probably works on n*x locally but I haven't tested it)
2. If you have text _under_ the list of latest posts on your homepage, you have at least 5 posts listed already (I could have put in some logic to deal with this but I am lazy)
3. You have `rsync` installed on your local machine
4. You have read and write permission for your gemlog files on the server, with the same username as your local machine (otherwise file permissions get weird)
5. You have permission to install files at `/usr/local/bin` on your local machine

## Installation

The easiest way to install `soyuz-cli` is using the install script. If you are using MacOS, run this command in `Terminal`:

```sh
curl https://hugh.run/install-soyuz | bash
```

Alternatively, you can build from source if you have rust and cargo installed.