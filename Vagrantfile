Vagrant.configure("2") do |config|
  config.vm.box_check_update = false
  config.vm.box = "ubuntu/bionic64"
  config.vm.synced_folder "./", "/shared"
  config.vm.network "forwarded_port", guest: 3000, host: 3000, protocol: "tcp"
  config.vm.provision "shell", inline: "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -", privileged: false
  config.vm.provision "shell", inline: "sudo add-apt-repository \"deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic stable\"", privileged: false
  config.vm.provision "shell", inline: "sudo apt-get update;sudo apt-get -y install dsniff stress curl build-essential iptables apt-transport-https ca-certificates curl software-properties-common docker-ce", privileged: false
  config.vm.provision "shell", inline: "sudo usermod -aG docker vagrant", privileged: false
  config.vm.provision "shell", inline: "curl https://sh.rustup.rs -sSf | sh -s -- -y", privileged: false
  config.vm.provision "shell", inline: "cargo install cross", privileged: false
end