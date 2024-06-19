variable "ssh_connection" {

    type = object({
        host            = string
        user            = string
        private_key     = string
    })

    sensitive = true

}

variable "managed_groups" {
  description = "List of managed groups with their configurations"
  type = list(object({
    name = string
    sudo = object({
      passwordless = bool
      commands = string
    })
  }))
}

variable "managed_users" {
  description = "List of managed users with their configurations"
  type = list(object({
    name = string
    create_home = bool
    groups = list(string)
    ssh_authorized_keys = list(string)
  }))
}