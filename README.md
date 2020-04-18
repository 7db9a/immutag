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

* bitcoin-sv - mutable file hierarchy and ledger

* git - collaborative file editing

* tmsu - local virtual filesytem with tag-based view

Each file is an address in a bitcoin HD wallet. The file is a directory on the device. A git directory is named after the file's bitcoin address.

```
immutag
├── 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG
│   ├── .git
│   ├── immutag-store
├── 1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt
│   ├── .git
│   ├── immutag-store

```
$ cat 1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG/immmutag-store

```
# IPFS-ADDR: TAGS, METADATA

QmeH81SYnASj5s91gQ22PdkYMgw45FD6kgrmBEU74Vp439: ["letter", "son"] , ""
QmQPRexanRL6pnSPAzC696if49BviGaLNKvC3gp3ApPQmN: ["letter", "son", "advice" ] , "How to be good and just."
```
Each ipfs address corresponds to specific file version. Above, there are 2 file versions, representing a single mutable file. The above is for conceptual purposes and the file may look a lot different.

### File branching

If a file version is considered important enough, a new branch can cheaply be created. A new hardened child bitcoin address is given to it. The forked file version is indicated by an opreturn on the original file. The opreturn has the forked file version's ipfs hash and the bitcoin address of the new branch.

### Bitcoin network

The user can push to bitcoin, which opreturns the latest ipfs addr of the file store.

### Private network

Collaborators can push and pull to the git repo. A user can reconstruct the git repo from the store entries.

### File discovery services

Users shouldn't be forced find immutags on bitcoin. Service providers should continuosly pull new immutags from the bitcoin network.  They can keep everything readily available in a database so that users can easily make queries.

## Development

### Setup

```
git clone https://github.com/7db9a/immutag
cd immutag
docker build immutag:0.1.0 .
docker volume create --name=immutag-cargo-data-volume
```

Install bsv. A bit awkward, but bear with for now.
```
docker run -it --name immutag -v $PWD:/immutag immutag:0.1.0
cd bsv && npm install --save-dev --save-exact
```

### Usage

Entry point.

`docker run -it --name immutag -v $PWD:/immutag immutag:0.1.0`

Playing with [moneybutton's bsv library](#moneybuttons-bsv) at the moment.

## Workflow, sort of.

There is no `immutag` app, yet. Just throwing ideas around.

$ immutag new IPFS-ADDR

Returns a bitcoin address, the first external one that is, which is immutable.

$ immutag update FIRST-ADDRESS IPFS-ADDR

Returns the latest external address if you update with a new file version.

A tagging solution can be layered on top. For example, TMSU can be used. In this case, files are named after immutable address. TMSU is then used to tag those files.

$ tmsu tag ADDRESS Faustina Afterlife

To find that file.

$ tmsu files video Faustina Afterlife

Returns immutable bitcoin addresses that correspond to tag 'video', 'Faustina', and 'Afterlife'.

$ immutag FIRST-ADDRESS

Returns the latest ipfs address of the latest file version.

$ ipfs cat $vidhash | mplayer -vo xv -

Plays video, 'Saint Faustina's Visions of the Afterlife'.

$ immutag ls FIRST-ADDRESS

Returns all the IPFS addresses corresponding to the address, not just the most-recent.

## Handling directories or git projects.

Tar the directory then follow procedure as if it's a regular file.

## Why ipfs' mfs won't do, I think.

Why not just use ipfs' Mutable File System (mfs)? It's my understanding that nodes must fully sync to share mfs data. It's okay for single users operating a single node, but that won't work for our use case. Nonetheless, we want to use ipfs.

If there's some way to use mfs among several nodes without fully syncing, or I'm misunderstanding something, I'd like to know.

## ipfs pegs on bitcoin network?

After incrementing the file versions some amount of times the ipfs addresses 'manifest' can be pegged to the bitcoin network. Users decide when to push, like on Github.

A file store is updated and referenced by ipfs address on each file update. That file, or its data, then becomes the latest ipfs 'peg'.

That file store is version controlled with git.

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

These database data can be 'pegged' to the bitcoin network. See [here](#ipfs-pegs-on-bitcoin-network).

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
