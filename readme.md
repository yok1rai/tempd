# Summary

You can auto-change the temperature using this user-level daemon

## Compatibility

currently it only works on hyprland, since it uses **HYPRSUNSET**, but *MAYBE* we will add cross-platform support

what u need

| Requirement | Minimum | Recommended |
|:------------|:--------|:------------|
| Electricity | Optional | Better electricity |
| Computer | ENIAC machine | ThinkPad |
| Linux | Required | Nyanarch Linux (just kidding) |
| Hyprland | Required | Hyprland with a good rice |
| hyprsunset | Required | idk what to put here |
| Init system | Yes | no systemd |
| D-Bus | Unfortunately | dbus-session |

## What does it do

it is a simple background tool that changes the screen temperature over time

there are 3 stages

- **Day:** 7:00 AM - 12:00 PM
- **Afternoon:** 12:00 PM - 7:00 PM
- **Night:** 7:00 PM - 7:00 AM

## how to install

js run

```bash
$ sudo chmod +x ./install
$ ./install
```

### i am a nerd tho

ok, if u are nerd then do these instead

1. make sure cargo is installed

```bash
$ cargo --version
```

if not install it

2. build the crate on release profile

```bash
$ cargo build --release
```

3. you must have it as `target/release/tempd`

```bash
$ ls target/release/tempd
target/release/tempd
```

if u get something else, then it means build was not succesfull, u can write an issue for it (or ask chatgpt idk)

4. make it executable

```bash
$ chmod +x target/release/tempd
$ ls -l target/release/tempd
-rwxr-xr-x 3.5M .......
```

5. move it to somewhere in ur PATH or somewhere u like, i'd advise `~/.local/bin`

```bash
$ cp target/release/tempd ~/.local/bin/
````

6. add it to ur hyprland config as exec-once

```bash
$ echo -e "\nexec-once = /home/$USER/.local/bin/tempd" >> ~/.config/hypr/hyprland.conf
```

7. exit and sign in

```bash
$ hyprctl dispatch exit
```

### license

it is licensed under MIT license

