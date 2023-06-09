variable "data_table" {}

resource "aws_iam_role" "lambda_role" {
  name = "ruby-yjit-3.2_lambda_role"
  assume_role_policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : "sts:AssumeRole",
        "Principal" : {
          "Service" : "lambda.amazonaws.com"
        },
        "Effect" : "Allow",
        "Sid" : ""
      }
    ]
  })
}

resource "aws_iam_policy" "lambda_policy" {
  name         = "ruby-yjit-3.2_battle_lambda_role"
  path         = "/"
  description  = "AWS IAM Policy for ruby yjit 3.2 lambda"

  policy = jsonencode({
    "Version": "2012-10-17",
    "Statement": [
      {
        "Action": [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ],
        "Resource": "arn:aws:logs:*:*:*",
        "Effect": "Allow"
      },
      {
        "Action": [
          "dynamodb:BatchGetItem",
          "dynamodb:GetItem",
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:BatchWriteItem",
          "dynamodb:PutItem",
          "dynamodb:UpdateItem"
        ],
        "Resource": var.data_table.arn,
        "Effect": "Allow"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "attach_iam_policy_to_iam_role" {
  role        = aws_iam_role.lambda_role.name
  policy_arn  = aws_iam_policy.lambda_policy.arn
}

locals {
  dist_path = "${path.module}/dist/lambda.zip"
}

data "archive_file" "source" {
  type = "zip"
  source_dir = "${path.module}/src"
  output_path = local.dist_path
}

resource "aws_lambda_function" "lambda" {
  filename      = local.dist_path
  function_name = "ruby-yjit-3_2_lambda"
  role          = aws_iam_role.lambda_role.arn
  handler       = "func.handler"

  source_code_hash = data.archive_file.source.output_base64sha256
  environment {
    variables = {
      "GEM_PATH" = "./vendor",
      "TABLE" = var.data_table.name,
      "RUBY_YJIT_ENABLE" = 1
    }
  }

  runtime = "ruby3.2"
  memory_size = 128
  timeout = 16
  #reserved_concurrent_executions = 1

  depends_on = [aws_iam_role_policy_attachment.attach_iam_policy_to_iam_role]
}

output "lambda" {
  value = aws_lambda_function.lambda
}
