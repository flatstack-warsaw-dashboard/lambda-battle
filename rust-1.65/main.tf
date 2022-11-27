variable "data_table" {}

resource "aws_iam_role" "iam_for_lambda" {
  name = "rust-1.65_lambda_role"
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
  name         = "ruby-2.7_battle_lambda_role"
  path         = "/"
  description  = "AWS IAM Policy for ruby 2.7 lambda"

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
  role        = aws_iam_role.iam_for_lambda.name
  policy_arn  = aws_iam_policy.lambda_policy.arn
}


resource "null_resource" "build_app" {
  provisioner "local-exec" {
    command = "cd ${path.module} && make prepare"
  }
}

locals {
  dist_path = "${path.module}/dist/lambda.zip"
}

data "archive_file" "source" {
  type = "zip"
  source_dir = "${path.module}/dist/src"
  output_path = local.dist_path
  depends_on = [null_resource.build_app]
}

resource "aws_lambda_function" "test_lambda" {
  function_name = "rust-1.65_lambda"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main"
  runtime       = "provided.al2"
  source_code_hash = data.archive_file.source.output_base64sha256

  depends_on = [
    data.archive_file.source,
    aws_iam_role_policy_attachment.attach_iam_policy_to_iam_role,
  ]

  environment {
    variables = {
      "TABLE" = var.data_table.name
    }
  }
}
