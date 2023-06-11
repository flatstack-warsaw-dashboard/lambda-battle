import * as cdk from "aws-cdk-lib";
import { type Construct } from "constructs";
import { Lambdas, ApiGateway, Database } from "./resources";

export class LambdaBattleStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const db = new Database(this, "Database");

    const lambdas = new Lambdas(this, "Lambdas", {
      baseTable: db.baseTable,
    });

    new ApiGateway(this, "ApiGateway", {
      lambdas: lambdas.all(),
    });
  }
}
