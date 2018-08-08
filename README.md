# Cheetah Vortex

A name generator from a source file.

## Quick start

Create a file, name it as you want (like `list_example.txt`).

You have to fill it with a potentially name on each line, like :

In `list_example.txt` :
```
alpha
beta
gamma
delta

# you can comment each line with an "#"

cheetah
lion

# empty lines are ignored

vortex
hurricane
```

Now download and launch the executable (like `cheetah-vortex.exe` on Windows) in a command-line :
```cmd
> cheetah-vortex.exe list_example.txt
Cheetah Vortex 1.0.0
Source file : list_example.txt

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

You can restart the program, names are randomly choosen each time :

```cmd
> cheetah-vortex.exe list_example.txt
Cheetah Vortex 1.0.0
Source file : list_example.txt

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