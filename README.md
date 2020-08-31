# Cheetah Vortex

A name generator from a source file.

## Usage

```txt
cheetah-vortex 1.3.0

USAGE:
    cheetah-vortex [FLAGS] [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Path of the input file (which contains words) [default: ./list.txt]

FLAGS:
    -h, --about         Prints about information
        --help          Prints help information
    -s, --no-restart    Does not ask for "regenerate a new set" at the end (it closes the program directly)
    -q, --quiet         Only prompt generated result, without further text (except errors and restart)
    -V, --version       Prints version information

OPTIONS:
    -i, --items <items_per_row>           Number of items combined per row [default: 2]
    -r, --rows <rows_generation_limit>    Number of rows generated on each run [default: 10]
        --separator <separator>           The separator between words [default:  ]
```

## Quick start

Create a file, name it as you want (like `test.txt`).

You have to fill it with a potentially name on each line, like :

In `test.txt` :

```txt
alpha
beta
gamma
delta

# you can comment each line starting with an "#"

cheetah
lion

# empty lines are ignored

vortex
hurricane
```

Now download and launch the executable (like `cheetah-vortex.exe` for Windows) in a command-line :

```cmd
> cheetah-vortex.exe test.txt
Starting cheetah-vortex V1.3.0

Currently using input file : test.txt

lion cheetah
vortex gamma
hurricane delta
gamma gamma
beta cheetah
hurricane delta
cheetah vortex
alpha lion
gamma hurricane
lion beta
```

You can restart the program, names are randomly selected each time :

```cmd
> cheetah-vortex.exe test.txt
Starting cheetah-vortex V1.3.0

Currently using input file : test.txt

lion delta
beta vortex
lion delta
hurricane alpha
vortex hurricane
vortex vortex
delta alpha
hurricane delta
delta beta
delta vortex
```
