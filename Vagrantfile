
Vagrant.configure("2") do |config|
  config.vm.box = "hashicorp/precise64"
  config.vm.box_check_update = false
  config.vm.provision :shell, inline: 'curl www.rust-lang.org/rustup.sh | sudo bash'
end

# -*- mode: ruby -*-
# vi: set ft=ruby :
