# Immutag

Exploring a 'mutable' and taggable content-addressable file system. This is under initial development.

## Overview

There are several libraries for tagging files. A couple that standout are [TMSU](https://github.com/oniony/TMSU), [squaretag](https://github.com/mdom/squaretag), and [tagspaces](https://github.com/tagspaces/tagspaces). They makes it easy to find files. And it works great for content-addressable files, such as with [ipfs](https://github.com/ipfs).

However, if the 'file' is updated, how can it remain content-addressable and taggable? We want:

* No syncing between nodes that access the same 'mutable' files (rules out ipfs' mfs).

* Can be layered on top of any content-addressable file-system or file tagging solution.

* Allows for collaboration.

What provides an immutable address space, that also can 'immutably', or determinstically, mutate? Bitcoin. Well, not just bitcoin, but hierchical deterministic cryptographic keys. And bitcoin, for our use case, works locally and on network.

## Development

### Setup

```
git clone https://github.com/7db9a/immutag
cd immutag
docker build immutag:0.1.0 .
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

## How it may work, more or less.

The following applications and technologies (others can be dropped in their place) may prove useful:

* ipfs

* bitcoin-sv

* sqlite

* tmsu

Each bitcoin address corresponds to a file. The first external address is the immutable address of a file. Other versions of the file are external addresses 1 and above.

Locally, files are mapped to addresses in an indexed store, such as an sql database.

The key is first external address, which is immutable. The values can change and are the following:

* latest external address

* ipfs address of the latest file version

Jump to [here](#hd-wallet---bip-44) for more details.

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

## Bitcoin SV libraries

Bitcoin SV has cheap tx fees and is commited to not breaking APIs or systems reliant on the network. The stakeholders are also friendly to users storing metadata on-chain, unlike Bitcoin Core. However, it doesn't have as wide a user-base as Bitcoin Core at the moment.

### [moneybutton's bsv](https://github.com/moneybutton/bsv)

Looks good. Here's a really useful doc [link](https://docs.moneybutton.com/docs/bsv-hd-private-key.html).

### [keyring](https://github.com/BitbossIO/keyring)

It doesn't appear to be stable, yet

## sqlite

[rustqlite](https://crates.io/crates/rusqlite)

## config writer

[toml-edit](https://crates.io/crates/toml_edit)

## Immutag filesystem

The filesystem may be something like below, but not in practice.

The file-system wallet is like "root" on unix-like operating systems. There would be few file-system wallets to many file-version wallets.

These database data can be 'pegged' to the bitcoin network. See [here](#ipfs-pegs-on-bitcoin-network).

**BIP-32 file-system HD wallet database**
```
BITCOIN-ADDR # First external address.

# BIP-44 bitcoin addresses can be derived from the master private key.
# 'PRIVKEY' values below are for individual BIP-32 HD file-version wallets.
# Below are 3 HD wallets representing 3 files.

0: BITCOIN-ADDR, XPRIV
1: BITCOIN-ADDR, XPRIV
2: BITCOIN-ADDR, XPRIV
```

**BIP-32 file-version HD wallet database**
```
BITCOIN-ADDR # First external address.

# Bitcoin addresses can be derived from "wallets" private key and are the first external addresses.
# Each ipfs address corresponds to specific file version.
# Below are 3 file versions, representing a single mutable file.

0: BITCOIN-ADDR, IPFS-ADDR
1: BITCOIN-ADDR, IPFS-ADDR
2: BITCOIN-ADDR, IPFS-ADDR
```

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
