░█▀█░█▀▀░▀█▀░█▀█░░░░░▀█▀░█▀█░▀█▀░▀█▀░░░░
░█░█░█░░░░█░░█░█░▄▄▄░░█░░█░█░░█░░░█░░░░░ 
░▀▀▀░▀▀▀░░▀░░▀▀▀░░░░░▀▀▀░▀░▀░▀▀▀░░▀░░▄▀░

A minimalist pid 1 system.

When people think of inits, the one which comes to mind first is systemd.
Although it is controversial due to it being monolithic, a lot of people prefer it.
But in the past, I've wondered

## What's the difference between systemd and other inits?
 ###  Size,
  Systemd has one of the largest binaries that an init can have (with its tools),
  weighing about 30-40 MB. (For comparison, runit weighs about 4 MB)
 ###  Speed,
  Due to systemd's bigger size and heavier tooling, the loading times are also
  higher than those of SysVinit.
 ###  Attack surface
  Again, due to larger size of the binaries the possible amount of exploits /
  weaknesses increases exponentially.
 ###  Clarity
  Systemd is a giant monolith which does a lot of things at once.
  While it can be easier to use, it is significantly more obfuscated.

But still, that same runit, s6, SysVinit, even busybox init don't push the boundaries the same way that octo-init does.

## In which ways is octo-init BETTER than those inits
 ###  Size
    I know size again but the difference is massive. In the previous example we talked about
    runit's miniscule 4 MB size, well, octo-init is 23 KB in size, before stripping.
    (That can be pushed even further if using the minimal profile, which is 14 KB before stripping.).
    With stripping, octo-init can be pushed to a minimum 11 KB of size. (That's over 350 X smaller than runit)
 ###  Speed
    Due to octo-init's small size, leveraging of VFS and use of raw syscalls, it can be much quicker than runit.
 ###  Portability
    Octo-init is STATICALLY LINKED.
    Basically that means that octo-init is a standalone binary which does not require dynamic external libraries
    to work.
 ###  Manageability.
    The source code of the init is small, and well separated into modules.
    Adding a feature and testing it is much easier than an integration to other inits.
    Also the use of rust's enums and structs allowed for clear zero-cost abstractions.
 ###  Safety
    Due to octo-init being statically linked, it also means that chances of breaking due to something else going on are much lower.
    All of the bugs which are on the system can be fixed with code.
    (Also, utilizing ONLY the stack allowed to reduce the risks of failure even further)
 ###  Customization
    Octo-init does not come only in one flavor. There are 3 versions available:
      1. Fast (performance focused, is bigger than the other 2),
      2. Balanced (It's smaller than the "Fast" one, but also isn't much slower)
      3. Minimal (The smallest one, but, also the slowest one)
    
    (Each one can be stripped down even further by stripping)

Now, it would seem like octo-init is the best init out of all but, there are some issues.

## In which ways is octo-init WORSE than those inits
 ###  Age
    Octo-init is really new and could contain some edge-cases which haven't been found yet.
 ###  Community
    Stemming off the same problem as age, octo-init does not have any packages made specifically for it yet,
    nor does it have a distro which utilizes it. While I do believe that it will be used somewhere, problems may
    arise.
 ###  Lack of features
    As you know, systemd is a giant monolith, so, octo-init can not possibly do all that systemd does,
    it relies on other tools to do it.

Thank you for reading,

If you are interested in trying it out, you might also want to check the octo-ctl repository

-> (For now empty, I'll add it later.)