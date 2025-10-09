#!/bin/bash

# Git aliases setup for devcontainer
echo "Setting up Git aliases..."

# Basic shortcuts
git config --global alias.co checkout
git config --global alias.br branch
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

# Additional shortcuts you might have in zshrc
git config --global alias.ga 'git add'
git config --global alias.gc 'git commit'
git config --global alias.gp 'git push'
git config --global alias.gl 'git pull'

# Shell alias for 'g' command (add to bashrc)
echo "# Git shortcut" >> ~/.bashrc
echo "alias g='git'" >> ~/.bashrc

# Also add to current session
alias g='git'

echo "Git aliases setup complete!"
git config --global --list | grep alias