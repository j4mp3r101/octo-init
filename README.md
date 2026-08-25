Welcome to Octo-init!

Octo-init is a minimalist init system which:
  is written in pure no_std rust.
  is really small (under 50Kb)
  is extremely fast.

The whole design was made so that octo-init is extremely light-weight(as said earlier),
whilist octo-ctl is a bit heavier BUT allows for more control.

like that i could combine both speed and ease of use.

about how octo-init works:
    There are 4 main stages (1 is startup, 2 is spawning, 3 is reaping and re-spawning, 4 is shutdown)

    When first parsing in stage 1, it creates a certain "list" of tasks which then gets passed to stage 2.
    (ALSO MOUNTS FILE SYSTEMS BY ITSELF). Also creates the first symlinks.

    After that, in stage 2 it spawns the processes by parsing the files passed from stage 2.

    Using the tmpfs disk it can efficiently save processes as files (specifically symlinks)
    which point to the actual file on the disk, whilist their name is the pid of the process.

    In stage 3 the init listens for signals. When it "hears" that a child died it uses an
    O(1) lookup to find the symlink, quickly parses it and updates the name.

    After all of that is done stage 4 takes over and gracefully shuts down the system (If a signal to do so was passed)

There is also a tool called "octo-ctl" which i made for communication with octo-init.
Ill put the link here to the repo later.