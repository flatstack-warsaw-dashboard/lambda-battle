import { Construct } from "constructs";
import { TLambdas } from "./lambdas";
import { LambdaIntegration, RestApi } from "aws-cdk-lib/aws-apigateway";


interface ApiGatewayProps {
  lambdas: TLambdas
}

export default class ApiGateway extends Construct {    
  constructor(scope: Construct, id: string, props: ApiGatewayProps){
    super(scope, id);

    this.createLambdaBattleApi(props.lambdas);
  }

  private createLambdaBattleApi(funcs: TLambdas) {
    const api = new RestApi(this, "lambda-battle-api", {
      deploy: true,
      restApiName: `Lambda Battle Api`
    })

    Object.entries(funcs).forEach(([name, func]) => {
      const integration = new LambdaIntegration(func);

      api.root.addMethod('POST', integration)
    })
  }
}