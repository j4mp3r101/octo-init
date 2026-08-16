Welcome to Octo-init!

Octo-init is a minimalist init system which:
  is written in pure no_std rust.
  is really small (under 20Kb)
  is extremely fast.

Now about actual usability.

It creates a folder in /etc/ called "octo-init".
The folder contains "entries" and "enabled".

"enabled" contains symlinks to the entries

If you want to add a startup process you need to add a file to entries (can be anything)
BUT! the files are sorted by the first 3 characters from highest to lowest (so 999 will be execute before 001.).
(Also it support all utf8 so A-Z, a-z, 0-9...)

the syntax for an entry is simple.

PATH /your/path (Has to be an absolute path.) (Path to the executable.)
TYPE SOME_TYPE (DAEMON, WAITFOR, WAITFORD(broken), ONESHOT)
ARG your_arg (Just put an arg)
ENV YOUR=env (Same as arg but env.)

It uses syscalls so its really fast and uh,

thats it!