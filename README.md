# Slowloris

Slowloris is an attack designed to make a web server unavailable.
Several requests are sent and held open, thus exhausting server resources.


:warning: **It's important to keep in mind that this exercice has been realised for educational purposes.** :warning:

## Usage

You can simply test the project by following these steps:

```bash
git clone https://github.com/ArToXxFR/slowloris
cd slowloris/
cargo run -- <IP> <PORT>
```

Alternatively, you can make it persistent on your system with the following command:

```bash
git clone https://github.com/ArToXxFR/slowloris
cd slowloris/
cargo install --path .
```

Make sure that the '~/.cargo/bin' is in your path, or add it with:

```bash
echo 'PATH=$PATH:$HOME/.cargo/bin' >> ~/.bashrc #(or ~/.zshrc if you're using zsh)
source ~/.bashrc
```

You can now test it :

```bash
#For example
slowloris 192.168.1.22 80 -w 1000
```

:information_source: Only tested on Linux (Fedora).

## Todo

- [X] Add command line
- [ ] Split code in different modules
- [ ] Add more documentation in the command line help
- [ ] Implement multi-threading to gain perfomance



