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

# swap space, for compilation
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab

# install tree
sudo apt install tree -y

# install c compiler, linker
sudo apt install build-essential

# install node, for pm2
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
\. "$HOME/.nvm/nvm.sh"
nvm install 22
npm install -g pm2

# remember disable password ssh login
sudo vim /etc/ssh/sshd_config
# set PasswordAuthentication no
sudo systemctl restart ssh

# google current way to install RUST, 8/25 was:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
