locals {
    tf_backend    = jsondecode(
        get_env("TF_BACKEND")
    )
}

generate "backend" {
    path        = "backend.tf"
    if_exists   = "overwrite_terragrunt"
    contents    = templatefile("../terragrunt_templates/s3_backend.tpl", {
        bucket      = local.tf_backend.bucket
        region      = local.tf_backend.region
        key         = "profit_prophet/${basename(get_terragrunt_dir())}/terraform.tfstate"
        access_key  = local.tf_backend.access_key
        secret_key  = local.tf_backend.secret_key
    })
}
