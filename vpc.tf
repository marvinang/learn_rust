# Configure the HuaweiCloud Provider
provider "huaweicloud" {
  region      = "cn-north-1"
  domain_name = "zysshsu"
  access_key  = "CPBVU9YVMZHG5MC1T2OA"
  secret_key  = "SLB8fM1bYTb1zP04kWSeXcWMQqozqQ8EUxaumalt"
}

# Create a VPC
# resource "huaweicloud_vpc_v1" "example" {
#   name = "my_vpc"
#   cidr = "192.168.0.0/16"
# }
