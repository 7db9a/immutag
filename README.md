
## Bitcoin SV libraries

[bsv](https://github.com/moneybutton/bsv)

Looks good.

https://docs.moneybutton.com/docs/bsv-hd-private-key.html

https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki


[keyring](https://github.com/BitbossIO/keyring)

* Javascript

* Under development. Last commit 2/4/2020. Doesn't look ready.


## Bitcoin libraries

[keychain-manager](https://github.com/iost-official/keychain-manager)

Looks good.

## Notes

coin    account chain       address path
Bitcoin first   external    first   m / 44' / 0' / 0' / 0 / 0
Bitcoin first   change      first   m / 44' / 0' / 0' / 1 / 0
Bitcoin first   external    second  m / 44' / 0' / 0' / 0 / 1
Bitcoin second  external    first   m / 44' / 0' / 1' / 0 / 0

(im)mutag files are versioned as so.

coin    account chain       address path
Bitcoin first   version     first   m / 44' / 0' / 0' / 300 / 0
Bitcoin first   version     second  m / 44' / 0' / 0' / 301 / 0
Bitcoin first   version     third   m / 44' / 0' / 0' / 302 / 0


Say you have file.txt. Your create a bitcoin account for it:

m / 44' / 0' / 0' / 0 / 0

You then 'commit' file.txt to the following address:

m / 44' / 0' / 0' / 300 / 0

This is the commit hash:

(address + file.sha256()).sha256()

That hash is added to a key value store. The address is the key and the hash is the value.

Another key-value store saves the hash as they key and file.sha256() as the value.

To push to network, create an op-return memo of file.sha256().

Locally, the files should be named after the addresses. That way you search for file versions by descending the key hierarchy.
