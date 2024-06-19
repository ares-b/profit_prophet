terraform {
  required_providers {
    oci = {
      source = "oracle/oci"
      version = "5.46.0"
    }
  }
}

provider "oci" {
  tenancy_ocid  = var.api_authentification.tenancy_ocid
  user_ocid     = var.api_authentification.user_ocid
  fingerprint   = var.api_authentification.fingerprint
  private_key   = var.api_authentification.private_key
  region        = var.api_authentification.region
}
