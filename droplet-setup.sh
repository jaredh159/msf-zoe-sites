# login as root
adduser jared
usermod -aG sudo jared
su jared
mkdir ~/.ssh
vim ~/.ssh/authorized_keys # add public key

# login as jared, then..

# set up firewall
sudo ufw allow OpenSSH
sudo ufw enable

# remove password
sudo EDITOR=vim visudo
# add line: `jared ALL=(ALL) NOPASSWD:ALL`

# setup nginx
sudo apt update
sudo apt upgrade -y
sudo apt install nginx -y
sudo systemctl start nginx
sudo systemctl enable nginx
# check status
sudo systemctl status nginx
# allow nginx in firewall
sudo ufw allow 'Nginx Full'
sudo ufw status

# install tree
sudo apt install tree -y

# install c compiler, linker
sudo apt install build-essential

# 👍 remember disable password ssh login

# google current way to install RUST, 8/25 was:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
