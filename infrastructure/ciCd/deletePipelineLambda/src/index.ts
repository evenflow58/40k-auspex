import {
  CloudFormationClient,
  DeleteStackCommand,
  DeleteStackCommandInput,
  ListStackResourcesCommandInput,
  ListStackResourcesCommand,
  ListStackResourcesCommandOutput,
  waitUntilStackDeleteComplete,
  DescribeStacksCommandInput,
} from "@aws-sdk/client-cloudformation";
import { WaiterConfiguration, WaiterState } from "@aws-sdk/util-waiter";
import {GetPipelineCommand, GetPipelineCommandInput, GetPipelineStateCommandOutput, CodePipelineClient} from "@aws-sdk/client-codepipeline"

export const handler = async (event: { BranchName: string }): Promise<any> => {
  try {
    console.log("event", event);

    const client = new CodePipelineClient({ region: "us-east-1" });
    const getPipelineInput: GetPipelineCommandInput = {
      name: `monster-week-${event.BranchName}`
    };
    const command = new GetPipelineCommand(getPipelineInput);
    const { pipeline } = await client.send(command);

    if (!pipeline?.stages) {
      console.log("Empty pipeline.")
      return;
    }

    let stacksToDelete: string[][] = pipeline
      .stages
      .reverse()
      .reduce((stageAcc, stage) => {
        if (!stage.actions || stage.actions.length === 0) return stageAcc;

        const actions = stage
          .actions
          .reverse()
          .reduce((actionAcc, action) => {
            if (action.runOrder &&
                action.configuration?.ActionMode === "CHANGE_SET_EXECUTE" &&
                action.configuration?.StackName) {
              actionAcc[action.runOrder] = actionAcc[action.runOrder] ?? [];
              actionAcc[action.runOrder].push(action.configuration?.StackName)
            }

            return actionAcc;
          }, []);

        if (actions.length > 0) stageAcc = stageAcc.concat(actions);

        return stageAcc;
      }, [])
      .filter(stack => stack)
      .reverse();

    if (!stacksToDelete) {
      console.log("No stacks to delete");
      return;
    }
    
    const stackClient = new CloudFormationClient({ region: "us-east-1" });

    for(const stacks of stacksToDelete) {
      await Promise.allSettled(stacks.map(async (stack: string) => {
        console.log(`Deleting stack ${stack}`);
        const deleteStackCommandInput: DeleteStackCommandInput = {
          StackName: stack
        };

        const response = await stackClient.send(new DeleteStackCommand(deleteStackCommandInput));
        console.log(JSON.stringify(response));

        const waitUntilStackDeleteInput: DescribeStacksCommandInput = {
          StackName: stack,
        };

        const waiterConfig: WaiterConfiguration<CloudFormationClient> = {
          client: stackClient,
          maxWaitTime: 300,
        };

        let stackDeleteResult;

        do {
          stackDeleteResult = await waitUntilStackDeleteComplete(
            waiterConfig,
            waitUntilStackDeleteInput
          );

          console.log("Stack Results");
          console.log(JSON.stringify(stackDeleteResult));
 
          // if (stackDeleteResult.state !== WaiterState.RETRY) {
          //   console.log(JSON.stringify(stackDeleteResult));
          // }
        } while(stackDeleteResult.state === WaiterState.RETRY);

        return Promise.resolve();
      }));
    }
  } catch (e) {
    console.log("It broke", e);
    throw e;
  }
}
