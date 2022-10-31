# Easy-alias

Assign numbers as command aliases.

# Motivation
Repeating commands can become tedious and error-prone, especially if they are long. Assigning regular aliases is a good alternative, but 2 things are worth mentioning about this approach:
1. You have to memorize the aliases
2. You may end up forgetting the actual commands since you are using only their aliases

# Usage

Store your commands in /home/$USER/aliases. Example:

docker images

docker container ls -a

docker system prune

docker volume prune

kind create cluster

kubectl get all

kind delete cluster

clear


# Run

    EASY-ALIAS - A CLI Alias manager 
    Authors: Lorenzo Costa <http://www.costa86.tech>
    Version: 1.0.0
    License: AGPL-3.0-or-later
    Crafted with ❤️ using Rust language
    Set the commands in /home/$USER/aliases
    
    Run a command based on its index:   
    
    0 - "clear"
    1 - "docker images"
    2 - "docker container ls -a"
    3 - "docker system prune"
    4 - "docker volume prune"
    5 - "kind create cluster"
    6 - "kubectl get all"
    7 - "kind delete cluster"


# Install

## Cargo

    cargo install easy-alias

## Ready-to-use executable

[./easy-alias](easy-alias)