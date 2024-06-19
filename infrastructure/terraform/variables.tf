# Authentification

variable "api_authentification" {

    type = object({
        tenancy_ocid        = string
        user_ocid           = string
        fingerprint         = string
        private_key         = string
        region              = string
    })

    sensitive = true
}

# InfluxDb
variable "influxdb_init" {

    type = object({
        port            = number
        user            = string
        password        = optional(string)
        organization    = string
        bucket          = string
    })

    sensitive = true
    
    default = {
        port            = 8086
        user            = "terraform"
        organization    = "project_z"
        bucket          = "profit_prophet"
    }
}

variable "influxdb_users" {
    type = list(object({
        name        = string
        password    = string
        is_admin    = bool
        grant       = object({
            database    = string
            privilege   = string
        })
    }))

    sensitive = true

    validation {
        condition     = alltrue([for user in var.influxdb_users : alltrue([for grant in user.grants : contains(["write", "read", "all"], grant.privilege)])])
        error_message = "Each privilege must be one of 'write', 'read' or 'all'."
    }

    default = []
}
