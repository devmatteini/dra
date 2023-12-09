# Assets

To generate the `demo.gif` you need to install [vhs](https://github.com/charmbracelet/vhs):

```shell
# NOTE: this commands works on linux

dra download -a -i -o ~/.local/bin charmbracelet/vhs
# vhs needs ttyd and ffmpeg
dra download -s ttyd.x86_64 tsl0922/ttyd && chmod +x ttyd.x86_64 && mv ttyd.x86_64 ~/.local/bin/ttyd
sudo apt install ffmpeg
```

Then you can modify [demo.tape](./demo.tape) and run

```shell
vhs demo.tape && open demo.gif
```
