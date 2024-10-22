# Boron
Stateless password manager powered by Argon2 and Hmac-Sha256. The core program and all plugins are to be specified.

## Philosophy
The plugin system for this password manager is to be as seamless as possible. In that all an additional plugin will do is add an extra way to encode your key. For example turning your 32-byte key into a Bitcoin Mnemonic or a PGP key. From a GUI perspective when encodig your key to your desired format you will see a drop down, once you selected your desired format (plugin), a config may appear.

A plugin can be broken down into 2 parts:
- The config
- The encoded output



## Plugins
- [x] Passwords
- [ ] BIP39
- [ ] OpenPGP

## Todo
- [ ] CLI
- [ ] GUI