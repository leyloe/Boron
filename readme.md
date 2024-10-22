# Boron
A stateless password manager powered by Argon2 and HMAC-SHA256. The core implementation and all plugins are to have a specification.

## Philosophy
The plugin system for this password manager is designed to be as seamless as possible. Each plugin simply adds an additional method for encoding your key. For instance, it could convert your 32-byte key into a BIP39 Mnemonic or a PGP key. From a GUI perspective, when encoding your key into a specific format, a drop-down menu will appear. After selecting the desired format (plugin), a configuration panel may appear based on the plugin’s requirements.

A plugin can be broken down into 2 parts:
- The config
- The encoded output

The config might be needed if you're working with something like passwords, you may need to specify your length, what characters to include, etc.

## Plugins
- [x] Passwords
- [ ] BIP39
- [ ] OpenPGP

## Todo
- [ ] CLI
- [ ] GUI