import { StackProps } from "aws-cdk-lib";
import { Code, Runtime, Function, IFunction } from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { ITable } from "aws-cdk-lib/aws-dynamodb";

enum ELangCase {
  Ruby2_7 = 'ruby-2-7-x86'
};

interface ILambdasProps extends StackProps {
  baseTable: ITable;
}

type TLambdaOptions = {
  runtime: Runtime,
  lpackage: string,
  env?: { [key: string]: string }
}

export type TLambdas = {
  [ELangCase.Ruby2_7]: IFunction
}


export default class Lambdas extends Construct {
  public static readonly PACKAGE_PATH = "./packages/"
  public static packagePathFor = (zip: string) => Lambdas.PACKAGE_PATH.concat(zip)
  
  public static readonly RUBY_LAMBDA_CONFIGS = {
    [ELangCase.Ruby2_7]: {
      runtime: Runtime.RUBY_2_7, 
      lpackage: Lambdas.packagePathFor('ruby-2.7.zip'),
      environment: {
        GEM_PATH: './vendor'
      }
    }
  }
  
  readonly Ruby2_7Lambda: Function;
  
  private props: ILambdasProps;
  
  constructor(scope: Construct, id: string, props: ILambdasProps) {
    super(scope, id);
    
    this.props = props
    this.Ruby2_7Lambda = this.createRubyLambda(ELangCase.Ruby2_7)
  }
  
  
  public all(): TLambdas {
    return { 
      [ELangCase.Ruby2_7]:  this.Ruby2_7Lambda 
    }
  }
  
  private createRubyLambda(version: ELangCase) {
    const config: TLambdaOptions = Lambdas.RUBY_LAMBDA_CONFIGS[version]
    
    const lambdaProps = {
      functionName: `${version}-Battle-Function`,
      code: Code.fromAsset(config.lpackage),
      handler: 'src/func.handler',
      runtime: config.runtime,
      environment: {
        GEM_PATH: './vendor',
        TABLE: this.props.baseTable.tableName,
        ...(config.env || {})
      }
    }
    
    const rubyFunction = new Function(this, `${version}-lambda`, lambdaProps);
    
    this.props.baseTable.grantReadWriteData(rubyFunction);
    
    return rubyFunction
  }
}
