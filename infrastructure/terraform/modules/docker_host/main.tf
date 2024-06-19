resource "null_resource" "ansible_provisioner" {

  # Wait for SSH Agent
  provisioner "remote-exec" {
    connection {
      type        = "ssh"
      host        = var.ssh_connection.host
      user        = var.ssh_connection.user
      private_key = var.ssh_connection.private_key
    }

    inline = ["echo 'connected!'"]
  }

  provisioner "local-exec" {
    environment = {
      ANSIBLE_CONFIG = "${path.root}/../ansible/ansible.cfg"
    }
    command = <<EOT
        # Write private key to a file
        echo "${var.ssh_connection.private_key}" > private_key_${var.ssh_connection.host}.pem
        chmod 600 private_key_${var.ssh_connection.host}.pem
        
        # Define the variables to pass to Ansible
        VARIABLES_JSON='{
          "managed_groups": ${jsonencode(var.managed_groups)},
          "managed_users": ${jsonencode(var.managed_users)}
        }'
        echo "$VARIABLES_JSON" > variables_${var.ssh_connection.host}.json

        # Run the Ansible playbook with verbose logging
        ansible-playbook -u ${var.ssh_connection.user} \
        --private-key private_key_${var.ssh_connection.host}.pem \
        -i ${var.ssh_connection.host}, \
        -e @variables_${var.ssh_connection.host}.json \
        ${path.root}/../ansible/configure_docker_host.yml
        
        # Capture the exit status
        ANSIBLE_EXIT_STATUS=$?

        # Clean everything
        

        # Exit with the Ansible exit status
        exit $ANSIBLE_EXIT_STATUS
        EOT
  }

}
