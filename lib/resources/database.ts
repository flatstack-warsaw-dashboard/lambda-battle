import { AttributeType, BillingMode, ITable, Table } from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export default class Database extends Construct {
  public readonly baseTable: ITable;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    this.baseTable = this.createBaseTable();
  }

  private createBaseTable(): ITable {
    const baseTable = new Table(this, 'lambda-battle-data', {
      tableName: 'lambda-battle-data',
      partitionKey: {
        name: 'langCase',
        type: AttributeType.STRING
      },
      sortKey: {
        name: "iteration",
        type: AttributeType.NUMBER
      },
      billingMode: BillingMode.PAY_PER_REQUEST
    });

    return baseTable;
  }
}