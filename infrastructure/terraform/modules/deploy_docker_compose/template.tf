data "template_file" "docker_compose" {
  template = file("${path.module}/templates/docker-compose.yml.tpl")

  vars = {
    services = jsonencode(var.docker_compose.services)
    networks = jsonencode(var.docker_compose.networks)
    volumes  = jsonencode(var.docker_compose.volumes)
  }
}
