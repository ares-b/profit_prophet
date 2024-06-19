resource "null_resource" "deploy_docker_compose" {
  connection {
      type        = "ssh"
      host        = var.ssh_connection.host
      user        = var.ssh_connection.user
      private_key = var.ssh_connection.private_key
  }

  provisioner "remote-exec" {
    inline = [ 
      "sudo mkdir -p ${var.docker_compose.remote_path}",
      "sudo chgrp $(groups ${var.ssh_connection.user} | awk '{print $3}') ${var.docker_compose.remote_path}",
      "sudo chown ${var.ssh_connection.user} ${var.docker_compose.remote_path}"
     ]
  }

  provisioner "file" {
    content      = data.template_file.docker_compose.rendered
    destination = "${var.docker_compose.remote_path}/docker-compose.yml"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo docker compose -f ${var.docker_compose.remote_path}/docker-compose.yml up -d"
    ]
  }

  triggers = {
    source_path = data.template_file.docker_compose.rendered
  }
}