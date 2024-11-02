# bevy-devcontainer-template

This is a template for setup bevy inside a devcontainer with a windows 11 host througth WSL 2.
`
## Docker desktop

install docker desktop : https://www.docker.com/products/docker-desktop/

## WSL2 Ubuntu

An Ubuntu WSL2 is necessary for share your DISPLAY by this way bevy can create a window.

- open a windows terminal
    - `wsl.exe --install` : install a WSL2 Ubuntu
        - validate the restart of docker-desktop
        - choose `dev` as username 
    - `wsl.exe --set-default Ubuntu` : Ubuntu as default WSL2 distro
    - `wsl.exe --list -v`  : check that Ubuntu is the default, '*' should  be on Ubuntu line
- In docker-desktop you should activate the WSL Integration (Settings/Ressources/WSL Integration)

## VScode

- open this repo with vscode
    - install this extensions
        - devcontainer
        - docker
        - WSL
- open the devcontainer
    - open a vscode terminal
        - `cargo run` : at the end of the build a black window should be displayed