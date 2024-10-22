# Boron
A stateless password manager powered by Argon2 and HMAC-SHA256. The core implementation and all plugins are to have a specification.

## Philosophy
The plugin system for this password manager is designed to be as seamless as possible. Each plugin simply adds an additional method for encoding your key. For instance, it could convert your 32-byte key into a BIP39 Mnemonic or a PGP key. From a GUI perspective, when encoding your key into a specific format, a drop-down menu will appear. After selecting the desired format (plugin), a configuration panel may appear based on the plugin’s requirements.

A plugin can be broken down into 2 parts:
- The config
- The encoded output

The config might be needed if you're working with something like passwords, you may need to specify your length, what characters to include, etc. However, the goal is to provide the best configuration defaults as possible so you don't have to remember too much.

In addition for added convenience you may have the option to backup and import your entries (label + username) so you don't have to input them every time. However your entries secrets will never be stored (not even encrypted), so you still need your master password. But you can have the option of encrypting the entries with a password.

As a reminder entries only consist of a label (website or service) + an optional username. The secrets will only be derived after you have inputted your master password.

## Plugins
- [x] Passwords
- [ ] BIP39
- [ ] OpenPGP

## Todo
- [ ] CLI
- [ ] GUI