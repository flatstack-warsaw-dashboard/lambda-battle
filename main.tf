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
  hash_key     = "langCase"
  range_key    = "iteration"

  attribute {
    name = "langCase"
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

module "ruby-yjit-3_2-lambda" {
  source = "./ruby-yjit-3.2"
  data_table = aws_dynamodb_table.data
  depends_on = [aws_dynamodb_table.data]
}

resource "aws_apigatewayv2_api" "gateway" {
  name          = "lambda-battle-api"
  protocol_type = "HTTP"
}

locals {
  lambda_resources = {
    "ruby-2_7-x86" = { "module" = module.ruby-2_7-lambda }, 
    "ruby-yjit-3_2-x86" = { "module" = module.ruby-yjit-3_2-lambda }
  }
}

resource "aws_lambda_permission" "invoke_permission" {
  for_each = local.lambda_resources
  action        = "lambda:InvokeFunction"
  function_name = each.value.module.lambda.arn
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.gateway.execution_arn}/*/*"
}

resource "aws_apigatewayv2_stage" "default" {
  api_id = aws_apigatewayv2_api.gateway.id
  name   = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "lambda_integration" {
  for_each = local.lambda_resources
  api_id           = aws_apigatewayv2_api.gateway.id
  integration_type = "AWS_PROXY"

  connection_type           = "INTERNET"
  description               = "${each.key} lambda integration"
  integration_method        = "POST"
  integration_uri           = each.value.module.lambda.invoke_arn
}

resource "aws_apigatewayv2_route" "lambda_route" {
  for_each = local.lambda_resources
  api_id    = aws_apigatewayv2_api.gateway.id
  route_key = "POST /${each.key}"

  target = "integrations/${aws_apigatewayv2_integration.lambda_integration[each.key].id}"
}

output "api_gateway" {
  value = {
    "gateway_url" = aws_apigatewayv2_api.gateway.api_endpoint
  }
}
