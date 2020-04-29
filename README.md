# Immutag

Exploring a 'mutable' and taggable content-addressable file system. This is under initial development.

There are several libraries for tagging files. A couple that standout are [TMSU](https://github.com/oniony/TMSU), [squaretag](https://github.com/mdom/squaretag), and [tagspaces](https://github.com/tagspaces/tagspaces). They makes it easy to find files. They are useful for keeping track of content-addressable files, such as with [ipfs](https://github.com/ipfs).

However, if the 'file' is updated, how can it remain content-addressable and taggable? IPFS' Mutable File System is meant only for a single node, if I'm not mistaken.

We want:

* No syncing between nodes that access the same 'mutable' files.

* Can be layered on top of any content-addressable file-system or file tagging solution.

* Collaborative.

What provides an immutable address space, that also can 'immutably', or determinstically, mutate? Bitcoin. Well, not just bitcoin, but hierchical deterministic cryptographic keys. And bitcoin, for our use case, works locally and on network.

## How it works - possibly

The following applications and technologies may prove useful:

* ipfs - immutable content-addresable files

* bitcoin - mutable file hierarchy and ledger

* git - collaborative file editing

* recfiles - tags stored in vcs (e.g. git) friendly file format

* tmsu - local virtual filesytem with tag-based view

These should be easily swappable with alternatives.

### Directory structure

You can have many filesystems. They're all in the `.immutag` directory located by default in $HOME, unless specified otherwise. Each filesystem is named after the first address in a bitcoin HD wallet. A git directory is named after the filesystem's bitcoin address.

```
.immutag
├── 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG
│   ├── .git
│   ├── version-store
│   ├── metadata
├── 1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt
│   ├── .git
│   ├── version-store
│   ├── metadata
├── immutag.toml

```

### Immutag file

All you need is an immutag.toml to initialize an immutag filesystem. The options `ledger`, and so forth, is how to extend the fileystem format to different protocols.

$ cat immutag/immutag.toml

```
['immutag']
version = "0.1.0"
ledger = "bitcoinsv"
contentsys = "ipfs"
filehash = "sha256"
vcs = "git"
```

Let's add some filesystems. The other entries represent complete fileystems.

$ cat immutag/immutag.toml

```
['immutag']
version = "0.1.0"
ledger = "bitcoinsv"
contentsys = "ipfs"
filehash = "sha256"
vcs = "git"


['1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG']
xpriv = "xprv9s21ZrQH143K29TJGFSiEAAQM8SMBH2V6x5Aaf9bqvXftrs1v274STWWKfz8svukBLGEQgWqkgRhpt2CNFY89CFaqdsA3gicZeqexk2itxf"
mnemonic = "certain dust pave crane renew multiply stone stuff proud flee fancy knee"


['1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt']
xpriv = "xprv9s21ZrQH143K3w3ZiXq14u2Ln2xp5wLjSmx8ypGrvhZ7rS7TKuFeQCviiwy1ULB51tkzWwbqHPFSBwyPJnFXkvH6U1RBgqaBeMEPJ4QZ9ov"
mnemonic ""legal winner thank year wave sausage worth useful legal winner thank yellow"
```

The keys are bitcoin public address entry points into the respective fileystems. Each filesystem is seperate HD wallet.

### Version store

$ cat 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/version-store

```
# file-hash: file-addr

1103def0e9d9036f59b7ef8524791710ed9a6e477b611abb94b0302edf887ee9: {bitcoin-file-addr (not version addr)}
f909e48c4b5b8aeaf45cd6844994b37a0de5c52d43b36410c35d9dd8ae6f9afb: {bitcoin-file-addr (not version addr)}
```
Every file version is keyed by filehash. The file address of each version is the value.

### Metadata

The metadata layer is seperate to avoid vendor-lock. Here we use recfile format, but some alternative can be used.

$ cat 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/metadata

```
# _*_ mode: rec _*_

%rec: Metadata
%mandatory: filehash version-addr content_addr file_type

file_hash: f909e48c4b5b8aeaf45cd6844994b37a0de5c52d43b36410c35d9dd8ae6f9afb
version_addr: 37gsHDLSG5TJvApGfiUZDaDo9mSr6rjLv6
content_addr: QmQPRexanRL6pnSPAzC696if49BviGaLNKvC3gp3ApPQmN
alias: lemonade_stand
alias_path: /home/me/corporate/sales/promos/
file_type: mp4
tag_video: 1
tag_sales: 1
tag_promotional: 1

filehash: 1103def0e9d9036f59b7ef8524791710ed9a6e477b611abb94b0302edf887ee9
version_addr: n2DoUfi8oUkTALKdd3AvVeTTyWg1AQmXCD
content_addr: QmQPRexanRL6pnSPAzC696if49BviGaLNKvC3gp3ApPQmN
file_type: mp4
alias: lemonade_stand
alias_path: /home/me/corporate/sales/promos/
metadata: "Our sales video about the best lemonade, ever."
tag_video: 1
tag_sales: 1
tag_promotional: 1
tag_lemonade: 1

# End of metadta.rec
```
Each version bticoin address is a child address of a Bitcoin hierarchical deterministic wallet. For more details, see [here](#file-versions).

### File branching

If a file version is considered important enough, a new branch can cheaply be created. A new hardened child bitcoin address is given to it. The forked file version is indicated by an opreturn on the original file. The opreturn has the forked file version's ipfs hash and the bitcoin address of the new branch.

### Bitcoin network

The user can push to bitcoin, which opreturns the latest ipfs addr of the file version-store.

### Private network

Collaborators can push and pull to the git repo. A user can reconstruct the git repo from the version-store entries.

### File discovery services

Users shouldn't be forced find immutags on bitcoin. Service providers should continuosly pull new immutags from the bitcoin network. They can keep everything readily available in a database so that users can easily make queries.

## Development

### Setup

```
git clone https://github.com/7db9a/immutag
cd immutag
docker build immutag:0.1.0 .
docker volume create --name=immutag-cargo-data-volume
```

### Usage

Launch.

`docker-compose up`

Test.

`./dev.sh rust test`

Test a specific case.

`./dev.sh rust test $name`

## Command Line Interface

### Init

`$ immutag init`

Creates `~/.immutag/immutag.toml`. For an alternate location, `--path`.

### Create a new or import an existing filesystem to immutag.

`$ immutag filesys new --alias $work --mnemonic $mnemonic`

The filesys data is saved to `~/.immutag`.

Make a bitcoin wallet with an app you like best. Immutag only needs the mnemonic. It can also use the master extended private key of the wallet, using `--xpriv` option, instead of `--mnemonic`.

### Add a new file

`$ immutag file new --ipfs-addr $ipfs-addr --filesys-alias $work`

**Nicknamed/aliased files**

`$ immutag file new my-file`

That'll add the file to the default filesystem. The file name will become the nickname in immutag.

### Aliases, add a new file

Name a file by its nickname. You could have added the file with another nickname, so it's not trivial.

`$ ls`

my-file

`$ immutag file new my-file --alias nickname-file`

`$ immutag file alias my-file

nickname-file

And from the full-path it was made from.

`$ immutag file alias --fullpath my-file`

/full/path/nickname-file

When using immutag against your files, say a git project, this is very handy. Even though its an alias, you'll much more easily recognize it rather than a ipfs or bitcoin address.

If you only have an ipfs address of the file version and prefer the non-default filesystem.

### Update a file

`$ immutag update --ipfs-addr IPFS-ADDR --filesys-alias $work`

**Using aliases/nicknames**

`$ immutag update my-file`

### Tag a file.

`$ immutag tag my-file showoff 2020`

### Find a file.

`$ immutag files showoff 2020`

alias    content-addr ledger-addr
my-file  $ipfs-addr   $bitcoin-addr

## Handling directories or git projects.

Tar the directory then follow procedure as if it's a regular file.

## Why ipfs' mfs won't do, I think.

Why not just use ipfs' Mutable File System (mfs)? It's my understanding that nodes must fully sync to share mfs data. It's okay for single users operating a single node, but that won't work for our use case. Nonetheless, we want to use ipfs.

If there's some way to use mfs among several nodes without fully syncing, or I'm misunderstanding something, I'd like to know.

## ipfs pegs on bitcoin network?

After incrementing the file versions some amount of times the ipfs addresses 'manifest' can be pegged to the bitcoin network. Users decide when to push, like on Github.

A file version-store is updated and referenced by ipfs address on each file update. That file, or its data, then becomes the latest ipfs 'peg'.

That file version-store is version controlled with git.

## Useful libraries

### BSV

#### [rust-sv](https://github.com/brentongunning/rust-sv)

#### [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin)

#### [nodejs - moneybutton's bsv](https://github.com/moneybutton/bsv)

For our purposes, some very relevant [documentation](https://docs.moneybutton.com/docs/bsv-hd-private-key.html).

### SQLITE

[rustqlite](https://crates.io/crates/rusqlite)

For a file-discover service prototype.

### Config file

[toml-edit](https://crates.io/crates/toml_edit)

## Bitcoin HD filesystem

The file-system wallet is like "root" on unix-like operating systems. Each file is associated with a bitcoin address.

The database data can be 'pegged' to the bitcoin network. See [here](#ipfs-pegs-on-bitcoin-network).

The first BIP-44 external address is reserved for the filesystem 'root'. Subsequent addresses are each files.

### File versions

The file versions use `m / 144,000'` hardened addresses, which are 'synthetic' and not used directly on the bitcoin network. The file system uses `m /44'`, so each file gets a real bitcoin address.

**The first file**

m / 44' / 0' / 0' / 0 / 1

**... and its first version.**

m / 144,000' / 1' / 0' / 0 / 0

**The second file**

m / 44' / 0' / 0' / 0 / 2

**... and its 3 versions.**

m / 144,000' / 2' / 0' / 0 / 0

m / 144,000' / 2' / 0' / 0 / 1

m / 144,000' / 2' / 0' / 0 / 2

`m / 144,000'` addresses are 'second layer'. At the moment, the described [pegging](#ipfs-pegs-on-bitcoin-network) with ipfs is the second layer. Perhaps some other protocol, cheaper than bitcoin, can be dropped-in for file versioning in the future.

### BIP 32 - bitcoin hierarchical deterministic wallet

https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki

### BIP 44 - bitcoin account hierarchy

```
coin    account chain       address path
Bitcoin first   external    first   m / 44' / 0' / 0' / 0 / 0
Bitcoin first   change      first   m / 44' / 0' / 0' / 1 / 0
Bitcoin first   external    second  m / 44' / 0' / 0' / 0 / 1
Bitcoin second  external    first   m / 44' / 0' / 1' / 0 / 0
```
https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
