Welcome to Octo-init!

Octo-init is an extremely minimalist init system inspired by rust.
The main unique traits of it being:
  written in 100% rust,
  no std library,
  utilizes raw syscals rather than using /bin/bash or other shells.
  uses a small, simple config file rather than a folder.

I mean all of the claims are pretty simple except the last one.
the "config file" was a decision made by me as its much easier to parse quickly.

The syntax is quite simple.
To add an entry you just have to write:
TYPE PATH\n
(\n is just means new line.)

there are 5 types you can use.
DAEMON, TTY, ONESHOT, WAITFOR, WAITFORD.

DAEMON -> The process is always up. if it crashes it restarts.
ONESHOT -> The process just starts, the init doesnt keep track of it and doesnt revive it.
TTY -> specifically for shells (/bin/sh or /bin/bash).
WAITFOR -> halts the init spawning loop until the waitfor task is finished.
WAITFORD -> is the same as waitfor. also halts BUT loops. (P.S. HALTS only when spawning the first time. After that it works like a daemon.)

If you looked at the code you can see that its neatly separated into stages.
each stage does its own job.

stage 1 -> Mounting.
stage 2 -> Spawning child procs, creating a list.
stage 3 -> Reaping and reviving cycle.
stage 4 -> Shutdown.

Its a chain reaction. in order for stage x to execute the previous one has to die.

for now the init is only working with x86_64(support for aarch and risc-v will be done in the future.)