resource "oci_identity_compartment" "alwaysfree_compartment" {
  compartment_id  = var.api_authentification.tenancy_ocid
  description     = var.compartment.description
  name            = var.compartment.name
}
