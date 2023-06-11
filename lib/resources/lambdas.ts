import { Duration, type StackProps } from "aws-cdk-lib";
import {
  Code,
  Runtime,
  Function,
  Tracing,
  type IFunction,
  type FunctionOptions,
  type AssetCode,
} from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { type ITable } from "aws-cdk-lib/aws-dynamodb";
import { RetentionDays } from "aws-cdk-lib/aws-logs";
import { ApiEventSource } from "aws-cdk-lib/aws-lambda-event-sources";

enum ELangCase {
  Ruby2_7 = "ruby-2-7-x86",
}

interface ILambdasProps extends StackProps {
  baseTable: ITable;
}

interface TCustomLambdaConfig {
  runtime: Runtime;
  code: AssetCode;
  handler: string;
  environment?: Record<string, string>;
}

export interface TLambdas {
  [ELangCase.Ruby2_7]: IFunction;
}

export default class Lambdas extends Construct {
  public static readonly DEFAULT_FUNCTION_PROPS: FunctionOptions = {
    logRetention: RetentionDays.ONE_WEEK,
    events: [ApiEventSource],
    timeout: Duration.seconds(60),
    tracing: Tracing.PASS_THROUGH,
  };

  public static readonly LAMBDA_CONFIGS = {
    [ELangCase.Ruby2_7]: {
      handler: "src/func.handler",
      code: Code.fromAsset("./packages/ruby-2.7.zip"),
      runtime: Runtime.RUBY_2_7,
      environment: {
        GEM_PATH: "./vendor",
      },
    },
  };

  readonly Ruby2_7Lambda: IFunction;

  private readonly props: ILambdasProps;

  constructor(scope: Construct, id: string, props: ILambdasProps) {
    super(scope, id);

    this.props = props;
    this.Ruby2_7Lambda = this.createBattleLambda(ELangCase.Ruby2_7);
  }

  public all(): TLambdas {
    return {
      [ELangCase.Ruby2_7]: this.Ruby2_7Lambda,
    };
  }

  private createBattleLambda(name: ELangCase): IFunction {
    const config: TCustomLambdaConfig = Lambdas.LAMBDA_CONFIGS[name];

    const battleLambdaProps = {
      ...Lambdas.DEFAULT_FUNCTION_PROPS,
      ...config,
      functionName: `${name}-Battle-Function`,
      environment: {
        TABLE: this.props.baseTable.tableName,
        ...(config.environment || {}),
      },
    };

    const battleLambda = new Function(
      this,
      `${name}-battle-lambda`,
      battleLambdaProps
    );

    this.props.baseTable.grantReadWriteData(battleLambda);

    return battleLambda;
  }
}
