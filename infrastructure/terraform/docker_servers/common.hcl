terraform {
  source = "${get_terragrunt_dir()}/../../modules/docker_server"
  after_hook "ansible" {
    commands = ["init-from-module", "init", "init-all" ]
    execute  = [
      "cp", "-R",
      "${get_parent_terragrunt_dir()}/../../ansible", "${get_working_dir()}"
    ]
  }
}
