#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { LambdaBattleStack } from '../lib/lambda-battle-stack';

const app = new cdk.App();
new LambdaBattleStack(app, 'LambdaBattleStack', {
});