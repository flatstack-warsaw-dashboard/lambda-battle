import { Construct } from "constructs";
import { TLambdas } from "./lambdas";
import { LambdaIntegration, RestApi } from "aws-cdk-lib/aws-apigateway";
import { IFunction } from "aws-cdk-lib/aws-lambda";


interface ApiGatewayProps {
  lambdas: TLambdas
}

export default class ApiGateway extends Construct {
  private battleApi: RestApi

  constructor(scope: Construct, id: string, props: ApiGatewayProps) {
    super(scope, id);

    this.createLambdaBattleApi(props.lambdas);
  }

  private createLambdaBattleApi(funcs: TLambdas) {
    this.battleApi ||= new RestApi(this, "lambda-battle-api", {
      deploy: true,
      restApiName: `Lambda Battle Api`
    })

    Object.entries(funcs).forEach(([k, w]) => this.addBattleLambdaEndpoint(k, w))
  }

  private addBattleLambdaEndpoint(name: string, func: IFunction) {
    const lambdaIntegration = new LambdaIntegration(func)

    this.battleApi.root
      .addResource(name)
      .addMethod("POST", lambdaIntegration)
  }
}