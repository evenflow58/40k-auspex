import {
  CloudFormationClient,
  DeleteStackCommand,
  DeleteStackCommandInput,
  waitUntilStackDeleteComplete,
  DescribeStacksCommandInput
} from "@aws-sdk/client-cloudformation";
import { WaiterConfiguration } from "@aws-sdk/util-waiter";

export const handler = async (event: { prefix: string, data: Array<string>}): Promise<any> => {
  console.log('event', event);

  const stackNames = event.data.map(( stackName: string ) => stackName);

  const deleteRequests = await Promise.allSettled(stackNames.map(deleteStack));

  if (deleteRequests.filter(({ status }) => status === 'rejected')) {
    return { result: 'RECHECK', prefix: event.prefix };
  }

  return { result: 'DONE' };
}

const deleteStack = async (stackName: string): Promise<void> => {
  const stackClient = new CloudFormationClient({ region: 'us-east-1' });
  const deleteStackCommandInput: DeleteStackCommandInput = {
      StackName: stackName
  };

  stackClient.send(new DeleteStackCommand(deleteStackCommandInput));

  const waitUntilStackDeleteInput: DescribeStacksCommandInput = {
    StackName: stackName,
  };

  const waiterConfig: WaiterConfiguration<CloudFormationClient> = {
    client: stackClient,
    maxWaitTime: 600000,
  };

  await waitUntilStackDeleteComplete(
    waiterConfig,
    waitUntilStackDeleteInput
  );
}