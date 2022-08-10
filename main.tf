terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 4.17.1"
    }
  }

  backend "s3" {
    bucket         = "fwd-remote-state"
    key            = "lambda-battle.tfstate"
    region         = "eu-central-1"
    encrypt        = true
    dynamodb_table = "tf-remote-state-locks"
    profile = "fwd-retro"
  }
}

provider "aws" {
  region = "eu-central-1"
  profile = "fwd-retro"

  default_tags {
    tags = {
      Provisioner = "Terraform"
      Project     = "Lambda-Battle"
    }
  }
}

resource "aws_dynamodb_table" "data" {
  name         = "lambda-battle-data"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "lang"
  range_key    = "iteration"

  attribute {
    name = "lang"
    type = "S"
  }

  attribute {
    name = "iteration"
    type = "N"
  }

  server_side_encryption {
    enabled = true
  }

  point_in_time_recovery {
    enabled = true
  }
}

module "ruby-2_7-lambda" {
  source = "./ruby-2.7"
  data_table = aws_dynamodb_table.data
  depends_on = [aws_dynamodb_table.data]
}
