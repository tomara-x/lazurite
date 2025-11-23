command line lapis

see the lapis readme for more info about it: [codeberg.org/tomara-x/lapis](https://codeberg.org/tomara-x/lapis)

## building

- install rust: https://www.rust-lang.org/tools/install
- on linux you need `libjack-dev` and `libasound2-dev` (`jack-devel` and `alsa-lib-devel` on void)
- clone
```
git clone https://codeberg.org/tomara-x/lazurite.git
cd lazurite
```
- build and run
```
cargo run --release
```

- or build and install
```
cargo install --path .
```

## plotting
adding `--features plot` to the cargo commands works the same as in lapis: [codeberg.org/tomara-x/lapis#plotting](https://codeberg.org/tomara-x/lapis#plotting)

## use with named pipes
```bash
# make a named pipe
mkfifo p
# run lazurite and make it read from that pipe
lazurite < p
```
from a different terminal
```bash
# anything written to that pipe will be evaluated
echo '2+2' > p
# write a whole file
cat file.rs > p
```

in vim you can make a visual selection and write it to the pipe
```
:w >> p
```

## thanks

- fundsp https://github.com/SamiPerttu/fundsp
- syn https://github.com/dtolnay/syn
- cpal https://github.com/rustaudio/cpal
- crossbeam_channel https://github.com/crossbeam-rs/crossbeam
- plotters https://github.com/plotters-rs/plotters

---

to the ones that can't be with us

