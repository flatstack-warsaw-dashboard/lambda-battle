import { Duration, StackProps } from "aws-cdk-lib";
import { Code, Runtime, Function, IFunction } from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { RetentionDays } from "aws-cdk-lib/aws-logs";
import * as ssm from 'aws-cdk-lib/aws-ssm';
import { ITable } from "aws-cdk-lib/aws-dynamodb";

enum OLangCase {
  Ruby2_7 = 'ruby-2-7-x86',
  Ruby3_2 = 'ruby-3-2-x86',
  Ruby3_2_YJIT = 'ruby-3-2-yjit-x86'
};

export interface LambdasProps extends StackProps {
  baseTable: ITable;
}

export type TLambdas = {
  [OLangCase.Ruby2_7]: IFunction,
  [OLangCase.Ruby3_2]: IFunction
  [OLangCase.Ruby3_2_YJIT]: IFunction
}

type LambdaOptions = {
  runtime: Runtime;
  lpackage: string;
  env?: { 
    [key: string]: string;
  }
}

export default class Lambdas extends Construct {
  public static readonly RUBY_LAMBDA_CONFIGS = {
    [OLangCase.Ruby2_7]: {
      runtime: Runtime.RUBY_2_7, 
      lpackage: 'ruby-2.7.zip',
      env: {
        GEM_PATH: './vendor'
      }
    },
    [OLangCase.Ruby3_2]: {
      runtime: Runtime.RUBY_3_2, 
      lpackage: 'ruby-3.2.zip'
    },
    [OLangCase.Ruby3_2_YJIT]: {
      runtime: Runtime.RUBY_3_2, 
      lpackage: 'ruby-3.2.zip',
      env: {
        'RUBY_YJIT_ENABLE': '1'
      }
    },
  }

  private Ruby2_7Lambda: Function;
  private Ruby3_2Lambda: Function;
  private Ruby3_2YJITLambda: Function;

  private props: LambdasProps;

  constructor(scope: Construct, id: string, props: LambdasProps) {
    super(scope, id);

    this.props = props

    this.Ruby2_7Lambda = this.createRubyLambda(OLangCase.Ruby2_7)
    this.Ruby3_2Lambda = this.createRubyLambda(OLangCase.Ruby3_2)
    this.Ruby3_2YJITLambda = this.createRubyLambda(OLangCase.Ruby3_2_YJIT)
  }

  public all = (): TLambdas => ({
    [OLangCase.Ruby2_7]:  this.Ruby2_7Lambda,
    [OLangCase.Ruby3_2]:  this.Ruby3_2Lambda,
    [OLangCase.Ruby3_2_YJIT]: this.Ruby3_2YJITLambda
  })

  private createRubyLambda(version: OLangCase) {
    const config: LambdaOptions = Lambdas.RUBY_LAMBDA_CONFIGS[version]

    const lambdaProps = {
      functionName: `${version}-Battle-Function`,
      code: Code.fromAsset(`./packages/${config.lpackage}`),
      handler: 'src/func.handler',
      runtime: config.runtime,
      environment: {
        TABLE: this.props.baseTable.tableName,
        ...(config.env || {})
      }
    }

    const rubyFunction = new Function(this, `${version}-lambda`, lambdaProps);

    this.props.baseTable.grantReadWriteData(rubyFunction);

    return rubyFunction
  }
}
