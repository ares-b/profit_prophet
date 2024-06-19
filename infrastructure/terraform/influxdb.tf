# Runs on docker_host_01_vm

# Influx DB Instance

resource "random_password" "influxdb_password" {
  length  = 16
  special = true
}

locals {
  influxdb_init_password = var.influxdb_init.password != null ? var.influxdb_init.password : random_password.influxdb_password.result
}

module "deploy_influxdb" {
  source             = "./modules/deploy_docker_compose"
  
  ssh_connection = {
    host        = module.docker_host_01_vm.alwaysfree_instance_ip
    user        = "terraform"
    private_key = module.docker_host_01_vm.alwaysfree_instance_privatekey
    public_key  = module.docker_host_01_vm.alwaysfree_instance_publickey
  }

  docker_compose = {
    remote_path = "/opt/influxdb"
    services    = [
      {
        name  = "influxdb"
        image = "influxdb:2.7.6-alpine"
        environment = [
          {
            name  = "DOCKER_INFLUXDB_INIT_MODE"
            value = "setup"
          },
          {
            name  = "DOCKER_INFLUXDB_INIT_USERNAME"
            value = var.influxdb_init.user
          },
          {
            name  = "DOCKER_INFLUXDB_INIT_PASSWORD"
            value = local.influxdb_init_password
          },
          {
            name  = "DOCKER_INFLUXDB_INIT_ORG"
            value = var.influxdb_init.organization
          },
          {
            name  = "DOCKER_INFLUXDB_INIT_BUCKET"
            value = var.influxdb_init.bucket
          }
        ]
        ports = [
          {
            from = 8086
            to   = 8086
          }
        ]
        networks = ["profit_prophet"]
        volumes  = ["/opt/influxdb/volumes:/var/lib/influxdb"]
      }
    ]
    networks = {
      profit_prophet = {
      }
    }
  }

  depends_on = [ 
    module.docker_host_01_docker
  ]
}

# Influx DB configure Users

# terraform {
#   required_providers {
#     influxdb = {
#       source = "DrFaust92/influxdb"
#       version = "1.6.1"
#     }
#   }
# }

# provider "influxdb" {
#   url       = "http://${module.docker_host_01_vm.alwaysfree_instance_ip}:${var.influxdb_init.port}"
#   username  = var.influxdb_init.user
#   password  = local.influxdb_init_password
# }

# resource "influxdb_user" "users" {
#   count     = length(var.influxdb_users)
#   name      = var.influxdb_users[count.index].name
#   password  = var.influxdb_users[count.index].password
#   admin     = var.influxdb_users[count.index].is_admin

#   grant {
#     database  = var.influxdb_users[count.index].grant.database
#     privilege = var.influxdb_users[count.index].grant.privilege
#   }
# }