locals {
    config = yamldecode(
        sops_decrypt_file("${get_terragrunt_dir()}/config.yaml")
    )
}

dependency "docker_server" {
    config_path = "${get_terragrunt_dir()}/../../docker_servers/${local.config.server}"
}

generate "private_key" {
    path        = "private_key.pem"
    if_exists   = "overwrite"
    contents    = dependency.docker_server.outputs.privatekey
    disable_signature = true
}

generate "provider" {
    path        = "provider.tf"
    if_exists   = "overwrite"
    contents    = templatefile("${get_terragrunt_dir()}/../../terragrunt_templates/docker_ssh_provider.tpl", {
        docker_server    = dependency.docker_server.outputs.ip
        private_key      = "${get_working_dir()}/private_key.pem"
    })
}

inputs = local.config

terraform {
  source = "${get_terragrunt_dir()}/../../modules/docker_service"
  before_hook "ansible" {
    commands = ["init", "plan", "apply"]
    execute  = [
      "chmod", "600", "${get_working_dir()}/private_key.pem"
    ]
  }
}


