---
author: Richard Snider (Trongle)
date: 2 October 2024
---
# Using OpenSSH like a pro
Welcome to my presentation!

Today we'll be talking about SSH (Secure SHell).

If you want to watch on your own device
and test out SSH now, open up your terminal
and type the following command
to watch this presentation
on your own computer
(should be available on all major operating systems):

```bash
ssh tress76.think.usu.edu -p 53531
```

But don't you dare peek ahead. >:(

---
# About this presentation
Today we'll be talking about:

* The SSH protocol
* Using OpenSSH
* SSH-related tools to supercharge your skillz
* Generating and using OpenSSH keys
* Configuring the OpenSSH client
* Configuring the OpenSSH server (briefly)

---
# One more thing

* Oh, and this presentation is open source!
* So is the software that I'm using to present.
* So are my other presentations for this club.
* Slideshow: https://github.com/CodeTriangle/fslc-presentations
* Software: https://github.com/maaslalani/slides

---
# Why SSH?
Imagine the following situation:
You own a server
(running a game, a website, or something else entirely).
You leave for a week
but keep the server up for your friends.
One day into your vacation,
you get a bunch of messages
about how the server broke.

In situations like these,
it would be nice to be able to log in
without having to actually be in the same room
as the computer.

Fortunately, people have been thinking this
since the dawn of computer networking
and have come up with several solutions.

---
# What SSH?
SSH is a protocol that allows you
to log in safely and remotely
to any computer running an SSH server.

SSH was created to replace other protocols
that had been used for similar purposes
but had notable drawbacks, including
`telnet`, `rlogin`, and `rsh`.

## What OpenSSH?
OpenSSH is the most popular implementation
of the SSH protocol
providing both a server and a client.
It is developed by the OpenBSD project
who are famous for emphasis on
code quality,
security,
and good documentation.

This presentation focuses
on interacting with OpenSSH,
although you're likely to find other SSH servers
such as Dropbear
running on memory-limited devices,
such as routers and wireless access points.

Read the manual pages:

```bash
man ssh
man ssh_config
man sshd
man sshd_config
```

---
# How SSH?
If you were here at the beginning
you may have already used SSH to start watching
this presentation.
If you didn't know,
you can watch a copy of this presentation
by running this command in a terminal:

```bash
ssh tress76.think.usu.edu -p 53531
```

However, even if you have that open,
you should open another terminal
and run this command:

```bash
ssh fslc@allmight.think.usu.edu
```

and use password `linux`.

## Single Command Mode

You can also run a single command and exit
by specifying it after the hostname

```bash
ssh fslc@allmight.think.usu.edu "cat trongle.txt"
```

---
# Syntax
The syntax of an SSH command,
in its most general form, is:

```bash
ssh [user@]hostname [-p port] [command]
```

Where square brackets represent optional components.

If the `user` is unspecified,
OpenSSH will by default try to log in
with your username
(the one that you're logged in as locally).

`hostname` can be a domain name
(as shown above)
or it can be an IP address.

SSH uses port 22 by default,
but `slides` uses a different port
so that it doesn't conflict with the system SSH,
so you have to specify the port.

The last component, the `command`,
is, as discussed, optional.
If included, SSH will run that command;
if not, it will run an interactive shell.

---
# Now what?
I installed a lot of fun stuff on the server.
Feel free to do some of the following things:

* Leave a message in `~/files` using `vim`, `emacs`, or `nano`.
* Read others' messages in `~/files`.
* `w`: to check who else is online.
* `htop`: to monitor the system.
* `figlet [message]`: to generate some ASCII art.
* `cowsay [message]`: to put words in the mouth of a cow.
* `cbonsai`: to grow a bonsai tree (`man cbonsai`).
* `tmatrix`: to look like an ep1c haxx0r.
* `confetty` or `confetty fireworks`: confetti and fireworks
* Play a game; there's `adventure`, `boggle`, `hack`, `tetris`, `hangman` and more.

If some of these graphical applications don't work,
try:

```bash
ssh -t fslc@allmight.think.usu.edu
```

---
# File transfer with scp
That's right, SSH also does file transfer.

Use the commands:

```bash
scp local-path user@hostname:remote-path
scp user@hostname:remote-path local-path
```

to transfer files to and from the remote machine, respectively.
Note that `local-path` and `remote-path`
can be absolute or relative.
If `remote-path` is a relative path,
it is relative to the home directory
of the remote user.

Note that this will not work
if the SSH server on the remote machine
does not support SFTP
or SFTP is not enabled.

You can also use the `sftp` command
for interactive file transfer.

## sshfs

This is something I use almost daily.
SSHFS is a Linux program
that can mount an SSH directory
on your local machine
to be accessed as if they were local.

```bash
sshfs fslc@allmight.think.usu.edu:files remote-files
```

---
# SSH keys
Now, this is all fine and good
but generally you don't *want* just anyone
to be able to remotely access your computer.
If you have an SSH server running
and you aren't careful about who you let in,
that can be very dangerous.

Obviously, in this case
I've just exposed the password to everyone
effectively killing any security measures
there are for this user.
But passwords aren't actually very secure either
because anyone can copy it down
if it's written anywhere
and then log into your user from anywhere.

The solution to this problem is SSH keys.
After you upload your SSH key to a server,
without the actual key file,
it is impossible to remotely log in as you.

Most good servers allow SSH logins
**only through SSH keys**.

---
# SSH keys

## Generating keys
Run this command to generate a good SSH key:

```bash
ssh-keygen -t ed25519
```

This will create a key using the ED25519 algorithm.

### Aside: key types

By default, SSH with generate a 2048-bit RSA key,
which is not very secure.
If you want a secure RSA key,
make sure it's 4096 bits or more
(use the `-b` option to achieve this).
Either way, ED25519 keys
are smaller and more secure.

## Uploading keys
If you have password access to the target machine,
you can run this command
to upload your key to the server

```bash
ssh-copy-id -i ~/.ssh/id_xxxxx.pub user@hostname
```

Make sure to fill in the `xxxxx` with a valid filename.
If you followed the above instructions,
it will be `id_ed25519.pub`.

If you don't have password access to the machine,
you will need to give your public key to a friend
who is already on the server and can add it manually,
or, in some cases,
upload it via a web interface.

---
# SSH keys

## Passphrases
While creating an SSH key,
you will be prompted for a passphrase.
This will basically lock the key
so that you have to type in the passphrase
before the key can be used to authenticate
with the server.

What gives?
I thought I didn't need a password anymore.
Well, passphrases are optional,
so you don't *need* one.
I'd still recommend it, though.
The chances of your private key being stolen
are much less than your password being stolen,
but just in case,
it doesn't hurt to have the security benefits
of both methods.

To change or remove your passphrase, run:

```bash
ssh-keygen -p -f ~/.ssh/id_xxxxx.pub
```

Again, remember to replace the `xxxxx`
with the actual name of your key.

---
# SSH keys

## Key management best practices
It is generally a good idea
to have one key per device.
That way, if one device is compromised,
only the key associated with that device
needs to be removed from the servers
and you won't need to create a new key
for every one of your devices

## Remove an SSH key
To remove a key from a device,
you'll need to open `~/.ssh/authorized_keys`
on the remote device
and hunt for the key matching the one
you want to remove.
Delete that line and save the file.
Done.

---
# OpenSSH configuration

OpenSSH has a robust configuration system
allowing for a wide variety of useful things.
To check yours, open up `~/.ssh/config`.

The OpenSSH configuration file make take the following structure:

```bash
Host allmight # Only apply when typing [user@]allmight
    Hostname allmight.think.usu.edu # Sets the Host to use this full domain name or IP address.
    User fslc # Log in as user "fslc" if no other user is set.

Host *.linux.usu.edu # Can also match with a wildcard.
    ...

Host * # Applies to all hosts
    ...
```

---
# SSH agent

The SSH agent is another convenience feature
provided by OpenSSH,
which allows you to store keys.
This can be helpful for a variety of reasons,
for instance
so that you don't need to repeatedly type their passphrases.

## Starting the agent

There are two ways to start `ssh-agent`:

To add it to your shell, run:

```bash
eval $(ssh-agent -s)
```

Then, when you're finished, you can run:

```bash
eval $(ssh-agent -s -k)
```

To launch it alongside a program (for example `tmux`), run:

```bash
ssh-agent tmux
```

## Using the agent

To add a key to the agent, run:
```bash
ssh-add # adds all keys with default names
ssh-add ~/.ssh/id_xxxxx.pub # adds specific key
```

To remove keys, do the following:

```bash
ssh-add -D # removes all keys with default names
ssh-add -d ~/.ssh/id_xxxxx.pub # removes specific key
```

---
# SSH agent

## Automatically add keys

Instead of adding keys via command
you can set a configuration option
that will immediately add them
whenever the SSH client
authenticates with that key, as so:

```bash
Host * # I recommend setting this for all clients.
    AddKeysToAgent yes
```

## Require lifetime for keys

You can make it so that
keys added to the SSH agent
expire after a certain amount of time,
avoiding a potential issue
where someone might use your keys
while you are away from your computer.

```bash
ssh-agent -t default-lifetime # set default lifetime for all keys
ssh-add -t [keyfile] # set lifetime of key upon adding
```

## Forward agent

You can make calls to a certain host
forward your SSH agent
so that you can use keys available
on the client side
for further connections.

```bash
Host allmight
    ForwardAgent yes
```

---
# SSH multiplexing

SSH multiplexing is another fun trick
that can cut out the need to retype passphrases
while also saving on resources.

When multiplexing,
SSH will direct all connections to a server
through one channel,
which also stays open
after closing the connection
so that future connections to that server
are also directed through this
already-authenticated channel.

This is configured as follows:

```bash
Host * # You may want to omit a few hosts for this one, if they're more sensitive.
    ControlMaster auto
    ControlPath ~/.ssh/cm-%r@%h:%p # Can be anything but should include at least %r, %h, and %p
    ControlPersist 30m # I recommend this. Drops connection after 30 minutes.
```

---
# tmux

I want to briefly introduce you
to another one of my favorite tools
(I actually use it on my own machine
all the time).

"tmux" stands for "Terminal Multiplexer"
and it allows you to open multiple terminals
side-by-side or in separate pages,
all nested within
a single actual terminal.

Invoking `tmux` is easy:

```bash
tmux        # to start a new session
tmux attach # to connect to an existing session
tmux a      # alias of the above
```
Then you can do a lot of things with it:

* `Ctrl-b %`: split pane vertically into two terminals
* `Ctrl-b "`: split pane horizontally into two terminals
* `Ctrl-b c`: create new "window" (think of them as pages or workspaces instead)
* `Ctrl-b &`: kill current window
* `Ctrl-b :`: run a tmux command
* `Ctrl-b d`: detach from the session (leaving it active but in the background)
* `Ctrl-b ?`: list all key bindings

You can also share a `tmux` session with a friend
by both attaching to the same session
at the same time.
I've done this before at work
when we all needed to alternate running different commands
but didn't want to share a keyboard
or move from our desks.

---
# SSH server

The OpenSSH server is often known
by the command used to initiate it,
`sshd`.

If you want to set up an SSH server of your own,
it is very easy.

On most GNU+Linux distros:

```bash
sudo systemctl enable --now sshd # maybe ssh
```

(if your distro does not have `systemd`,
you're on your own)

On Windows:

```powershell
Start-Service sshd # or, if that doesn't work:
Set-Service -Name sshd -StartupType 'Automatic'
```

On Mac:

```bash
sudo systemsetup -setremotelogin on # i think???
```

But be aware of the consequences!

---
# SSH server

## Warnings and advice

Password authentication is enabled
in the default configuration
for most SSH servers.
As discussed previously,
this can be dangerous
if someone gets their hands on your password.

For that reason,
I recommend editing your sshd config,
which is traditionally found
at `/etc/ssh/sshd_config`
on Unix-like systems.
You can disable it like so:

```bash
PasswordAuthentication no
```

If the device is truly a server,
open to the whole internet,
I would consider this
an absolute necessity.

If, however, your device is not likely
to be widely available,
for instance
if you choose to enable an SSH server
on your desktop or laptop,
where it will only be visible
to the handful of devices
on your local area network,
password authentication may be preferable
as you might want to log in
from various computers around your home
without worrying about giving everything a key.

I disable password authentication on my laptop
when on campus
just because I'm paranoid.

Check out the man page
to see what else you can do
with `sshd`.

---
# Port forwarding

Oh yeah, now we're getting into the esoteric stuff.

SSH allows you to reroute traffic
from a port on either machine
to another computer
on the opposite side.

## Local forwarding

```bash
ssh -L8080:localhost:5000 fslc@allmight.think.usu.edu
```

To break down this option, it looks a bit like this:

| Redirect traffic from local port | to the host-side server | at port |
|---|---|---|
| 8080 | localhost | 5000 |

This means that
if I make a request to port 8080 on my machine
the server will make a request to localhost:5000.

This is extremely useful
when the computers are on different networks
and you need to access the other network.
In our case, both of the computers
that we are using to demonstrate SSH
are on the same network,
I came up with a different demo.

---
# Port forwarding

## Remote forwarding

The syntax for this is similar:

```bash
ssh -R5000:localhost:8080 fslc@allmight.think.usu.edu
```

and it means:

| Redirect traffic from remport port | to the client-side server | at port |
|---|---|---|
| 5000 | localhost | 8080 |

---
# Port forwarding

## Dynamic port forwarding

Dynamic port forwarding
uses a protocol called SOCKS
to allow you
to connect through one port
to any number of servers
only accessible from the remote side.

```bash
ssh -D1337 fslc@allmight.think.usu.edu
```

Support is more limited
for this kind of port forwarding
since it is not transparently passed through,
but it is more versatile.

Most browsers support setting a SOCKS proxy,
many phones allow setting a system-wide SOCKS proxy,
and, if all else fails,
I have used `proxychains` before
to pipe a program through a proxy.

**PLEASE DO NOT** make a habit
of proxying all your traffic through allmight.

---
# Port forwarding

## Limitations of SSH port forwarding

Other port-forwarding solutions exist
and are more flexible.
For anything that needs to stay online
you will want to use a system service.

Port forwarding is also useful
for routing incoming traffic from the internet
to a server on your local network,
but you should use your router's port forwarding.

SSH port forwarding
is mainly useful
because it is fast to set up temporarily
and debug with.

---
# Use as a git server

Did you know that every computer with an SSH server
is also a git server?

Recall that to clone a repository from GitHub, you use the command:

```bash
git clone git@github.com:CodeTriangle/fslc-presentations
```

With the knowledge we have now,
we can now identify this
as an SSH connection URL
like one you'd use with `scp`.

What we can do is set up a git repository
on the server
and then clone, push, and pull
from that repository
just as we would with GitHub or GitLab.
It works just like a normal remote.

```bash
# add to existing repo
git remote add allmight fslc@allmight.think.usu.edu:path/to/git/repo
git pull allmight
git push allmight

# or clone
git clone fslc@allmight.think.usu.edu:path/to/git/repo
```

As with before,
the path component
can be a relative
or an absolute path.

(But if you're doing this on a large scale,
you probably just want to set up
your own self hosted git like Gitea.)

---
# Escape sequences

This is the most obscure topic I have for you today.

While in an SSH session,
press `Enter` then `~`.
This initiates an escape sequence.

The following escape sequences are defined:

* `~.`: disconnect
* `~#`: list forwarded connections
* `~C`: add or remove forwarded connection
* `~?`: list escape sequences
* `~^Z`: background SSH
