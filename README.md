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
# BITCOIN-ADDR: IPFS-ADDR, TAGS, METADATA

1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG: QmeH81SYnASj5s91gQ22PdkYMgw45FD6kgrmBEU74Vp439, ["letter", "son"] , ""
n2DoUfi8oUkTALKdd3AvVeTTyWg1AQmXCD: QmQPRexanRL6pnSPAzC696if49BviGaLNKvC3gp3ApPQmN, "letter", "son", "advice" ], "How to be good and just."
```
Each bitcoin address corresponds to specific file version. Above, there are 2 file versions, representing a single mutable file. The above is for conceptual purposes and the file may look a lot different.

Each version bticoin address is a child address of a Bitcoin hierarchical deterministic wallet. For more details, see [here](#file-versions).

Immutag searches for an "immutag-file" in the current directory.

```
['immutag']
version = "0.1.0"

['1LrTstQYNZj8wCvBgipJqL9zghsofpsHEG']
xpriv = "xprv9s21ZrQH143K29TJGFSiEAAQM8SMBH2V6x5Aaf9bqvXftrs1v274STWWKfz8svukBLGEQgWqkgRhpt2CNFY89CFaqdsA3gicZeqexk2itxf"
mnemonic = "certain dust pave crane renew multiply stone stuff proud flee fancy knee"


['1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt']
xpriv = "xprv9s21ZrQH143K3w3ZiXq14u2Ln2xp5wLjSmx8ypGrvhZ7rS7TKuFeQCviiwy1ULB51tkzWwbqHPFSBwyPJnFXkvH6U1RBgqaBeMEPJ4QZ9ov"
mnemonic ""legal winner thank year wave sausage worth useful legal winner thank yellow"
```

Each entry is a complete fileystem with potentially many respective files.

### File branching

If a file version is considered important enough, a new branch can cheaply be created. A new hardened child bitcoin address is given to it. The forked file version is indicated by an opreturn on the original file. The opreturn has the forked file version's ipfs hash and the bitcoin address of the new branch.

### Bitcoin network

The user can push to bitcoin, which opreturns the latest ipfs addr of the file store.

### Private network

Collaborators can push and pull to the git repo. A user can reconstruct the git repo from the store entries.

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

## Workflow, sort of.

There is no `immutag` app, yet. Just throwing ideas around.

Initializes immutag in the current directory by importing a mnemonic. Can also pass `--xpriv`. The user creates a wallet from a standard bitcoin wallet, then imports it here.

`immutag init --mnemonic $MEMONIC`

Returns a bitcoin address, the first external one that is, which is immutable. Each address represents a file.

`immutag new IPFS-ADDR`

Returns the latest external address if you update with a new file version.

`immutag update FIRST-ADDRESS IPFS-ADDR`

To find that file.

`tmsu tag ADDRESS Faustina Afterlife`

Returns immutable bitcoin addresses that correspond to tag 'video', 'Faustina', and 'Afterlife'.

`tmsu files video Faustina Afterlife`

Returns the latest ipfs address of the latest file version.

`immutag FIRST-ADDRESS`

Plays video, 'Saint Faustina's Visions of the Afterlife'.

`ipfs cat $vidhash | mplayer -vo xv -`

Returns all the IPFS addresses corresponding to the address, not just the most-recent.

`immutag ls FIRST-ADDRESS`


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
