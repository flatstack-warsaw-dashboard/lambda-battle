import { Duration, StackProps } from "aws-cdk-lib";
import { Code, Runtime, Function, IFunction } from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { RetentionDays } from "aws-cdk-lib/aws-logs";
import * as ssm from 'aws-cdk-lib/aws-ssm';
import { ITable } from "aws-cdk-lib/aws-dynamodb";

export interface LambdasProps extends StackProps {
  baseTable: ITable;
}

export type TLambdas = {
  Ruby2_7Lambda: IFunction
}

type LambdaOptions = {
  runtime: Runtime,
  lpackage: string
}

export default class Lambdas extends Construct {
  readonly Ruby2_7Lambda: Function;
  private props: LambdasProps;

  constructor(scope: Construct, id: string, props: LambdasProps) {
    super(scope, id);

    this.props = props
    this.Ruby2_7Lambda = this.createRubyLambda({
      runtime: Runtime.RUBY_2_7, lpackage: 'ruby-2.7.zip'
    });
  }

  public all = (): TLambdas => ({
    Ruby2_7Lambda: this.Ruby2_7Lambda
  })

  private createRubyLambda(opts: LambdaOptions) {
    const lambdaProps = {
      code: Code.fromAsset(`./packages/${opts.lpackage}`),
      handler: 'src/func.handler',
      runtime: opts.runtime,
      environment: {
        GEM_PATH: './vendor',
        TABLE: this.props.baseTable.tableName
      }
    }

    const rubyFunction = new Function(this, 'ruby-2_7_lambda', lambdaProps);

    this.props.baseTable.grantReadWriteData(rubyFunction);

    return rubyFunction
  }

}