locals {
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
                chomp(module.oracle_alwaysfree.alwaysfree_instance_publickey)
            ]
        },
        {
            name                = "terraform",
            create_home         = true,
            groups              = ["automated_ops", "docker"],
            ssh_authorized_keys = [
                chomp(module.oracle_alwaysfree.alwaysfree_instance_publickey)
            ]
        },
    ]
}