# Matrix

Matrix is an open network for secure, decentralized communication.
* That means it's a chat solution, like Discord.

Unlike Discord, there are many different clients.
* Anything that supports the Matrix protocol can connect.

Matrix is decentralized, meaning that it doesn't run on only one company's servers.
* Many people host their own Matrix server, and you can too!

Watch this presentation on your own device! Open a terminal and type*:

```sh
ssh tress76.think.usu.edu -p 53531
```

*yes, even on Windows

---
# Matrix servers

Matrix servers contain users and channels.

Servers are hosted by individuals or organizations.

Some common general-purpose Matrix servers:
* `matrix.org`
* `mozilla.org`

A lot of open-source projects host their own server as well:
* `gnome.org`
* `opensuse.org`
* `nixos.org`

There are also a lot of very specialized servers run by individuals:
* `glasgow.social`
* `kirbygang.com`
* `bsd.cafe`

---

# Matrix users

Each Matrix user is hosted on a single Matrix server (its *homeserver*).

The format of a Matrix username is: `@username:domain`.
* Ex. `@codetriangle:matrix.org` is hosted at `matrix.org`.
* Ex. `@trongle:linux.usu.edu` is hosted at `linux.usu.edu`.

It's not uncommon to have multiple matrix accounts for reliability.

(For instance, both of the accounts listed above can be used to reach me.)

---

# Matrix rooms and spaces

Matrix **rooms** are like Discord channels:
* You can send, react to, and respond to messages.
* Threads are supported to separate topics.

Matrix **rooms** are also unlike Discord channels:
* Rooms are not necessarily part of a larger structure like a Discord server.
* You must join each room individually.
* Users can use as many custom and animated emojis as they like.

Matrix **spaces** are like Discord servers:
* They contain a list of rooms.

Matrix **spaces** are unlike Discord servers:
* Multiple spaces can contain the same room.
* You still must join each channel individually.
* Spaces can contain other spaces.

---

# Federation

Matrix is a **distributed** or **federated** service.

Federation, in this context means:
* multiple servers (called *instances*) are connected, owned by different people.
* those instances communicate heavily among themselves.
* through that communication, the same service is available through any instance.

In the context of matrix, this means:
* a member of any matrix can join a room or space hosted on another instance.
* members of two different instances can privately chat.
* rooms or spaces can be added to a space on a different instance.

## Other federated services

The "fediverse" is the connected web of federated servies. You may have heard of:

* **Mastodon**: microblogging (Twitter alternative)
* **Lemmy**: news aggregator (Reddit alternative)
* **PeerTube**: video hosting (YouTube alternative)

---

# Encryption

Matrix supports **end-to-end encryption** for messages.
* Only you and your correspondants can read encrypted messages.

How it works:
* The first device that you register with a matrix account generates keys.
* The next time you log into your matrix account, you can "verify" the new device.
* This initiates a chain of key signing that enables the device to decrypt messages.
* Since your keys are only available on your devices, server admins cannot read them.

As long as you have at least one device signed in, you won't lose your encrypted data.
* You can also set up encryption recovery.
* This sends an encrypted copy of your keys to the server, unlocked via passphrase.
* This allows you to get your keys back even if you have no devices signed in.

(If someone wants to read the spec
and can explain matrix keysigning to me
in a way that I understand,
then I'll buy you lunch)

---

# Other reasons matrix is cool

* Open-source
* You can choose from many clients, i.e.:
  * element <https://app.element.io/>
  * cinny <https://app.cinny.in/>
  * fluffychat <https://fluffy.chat/>
  * gomuks <https://github.com/gomuks/gomuks/>
  * iamb <https://iamb.chat/>
  * and more <https://matrix.org/ecosystem/clients/>
* End-to-end encrypted
* Tracking is almost impossible
* No paywalls
* No forced AI features

Plus, after today, all your FSLC friends will be on it.

---

# Join the FSLC homeserver and space

1. Go to <https://linux.usu.edu/> and click "create account"
1. Proceed by logging into discord (this is just for account provisioning)
   * If you do not have a discord account, get in touch with me
1. Choose a name and display name and create your FSLC account
1. Add a passkey OR password and TOTP to your account
1. Navigate to <https://chat.linux.usu.edu> and log in if needed
   * You can also use another matrix client like <https://app.element.io>
   * ...but you will have to manually input linux.usu.edu as your homeserver
1. This should create your account and automatically add you to the FSLC space

Unless there are any questions, the time is now yours to chat!

Thanks for coming and I hope you enjoy yourself on matrix.

Once again, to view this presentation and read the slides, open a terminal and type:

```sh
ssh tress76.think.usu.edu -p 53531
```
