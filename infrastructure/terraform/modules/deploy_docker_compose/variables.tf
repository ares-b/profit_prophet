variable "ssh_connection" {

    type = object({
        host            = string
        user            = string
        private_key     = string
    })

    sensitive = true

}

variable "docker_compose" {
  type = object({
    remote_path = string
    services = list(object({
      name        = string
      image       = string
      environment = optional(list(object({
        name  = string
        value = string
      })), [])
      ports = optional(list(object({
        from = number
        to   = number
      })), [])
      networks = optional(list(string), [])
      volumes  = optional(list(string), [])
    }))
    networks = optional(map(object({
      driver      = optional(string)
    })), {})
    volumes = optional(list(string), [])
  })
}