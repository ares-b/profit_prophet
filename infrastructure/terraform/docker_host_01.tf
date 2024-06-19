module "docker_host_01_vm" {
  source             = "./modules/oracle_alwaysfree"
  
  api_authentification = var.api_authentification
  ingress_rules = [
    {
      protocol  = "6",
      source    = "0.0.0.0/0"
      min_port  = 8086
      max_port  = 8086
    },
    {
      protocol  = "6",
      source    = "0.0.0.0/0"
      min_port  = 22
      max_port  = 22
    }
  ]
}

module "docker_host_01_docker" {
  source     = "./modules/docker_host"

  ssh_connection = {
    host        = module.docker_host_01_vm.alwaysfree_instance_ip
    user        = module.docker_host_01_vm.alwaysfree_user
    private_key = module.docker_host_01_vm.alwaysfree_instance_privatekey
  }

  managed_groups = [
    {
      name = "automated_ops"
      sudo = {
        passwordless  = true
        commands      = "ALL"
      }
    }
  ]

  managed_users = [
    {
      name                = "ansible",
      create_home         = true,
      groups              = ["automated_ops"],
      ssh_authorized_keys = [
        chomp(module.docker_host_01_vm.alwaysfree_instance_publickey)
      ]
    },
    {
      name                = "terraform",
      create_home         = true,
      groups              = ["automated_ops"],
      ssh_authorized_keys = [
        chomp(module.docker_host_01_vm.alwaysfree_instance_publickey)
      ]
    },
  ]

  depends_on = [
    module.docker_host_01_vm
  ]
}
