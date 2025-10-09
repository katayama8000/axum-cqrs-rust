#!/bin/bash

# Git aliases setup for devcontainer
echo "Setting up Git configuration and aliases..."

# Git user configuration
git config --global user.name "katauama8000"
git config --global user.email "tattu.0310@gmail.com"

# Basic shortcuts
git config --global alias.co checkout
git config --global alias.b branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.sw switch

# Useful commands
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'

# Advanced aliases
git config --global alias.lg "log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"
git config --global alias.aliases 'config --get-regexp alias'

echo "alias g='git'" >> ~/.bashrc

# Also add to current session
alias g='git'

echo "Git aliases setup complete!"
git config --global --list | grep alias