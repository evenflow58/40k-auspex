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
import { v4 } from "uuid";

export const handler = async (event: { BranchName: string }): Promise<any> => {
  try {
    console.log("event", event);

    const id = v4();

    const client = new CodePipelineClient({ region: "us-east-1" });
    const getPipelineInput: GetPipelineCommandInput = {
      name: `monster-week-${event.BranchName}`
    };
    const command = new GetPipelineCommand(getPipelineInput);
    const { pipeline } = await client.send(command);

    const stackClient = new CloudFormationClient({ region: "us-east-1" });
    const stacksAlreadyDeleting: string[] = [];

    const stacksToDelete: string[] = [];

    pipeline?.stages
      ?.reverse()
      .forEach(async stage => {
        stage.actions
          ?.filter(stage => stage.runOrder)
          .reverse()
          .forEach(action => {
            if (
              action?.runOrder &&
              action.configuration?.StackName &&
              !stacksToDelete.includes(action.configuration?.StackName)) {

              stacksToDelete.push(action.configuration?.StackName);
            }
          })
          // .reduce((acc, action) => {
          //   if (action?.runOrder && action.configuration?.StackName) {
          //     if (!acc[action.runOrder]) acc[action.runOrder] = []

          //     acc[action.runOrder].push(action.configuration.StackName)
          //   }

          //   return acc;
          // }, [] as Array<Array<string>>)
          // .forEach(async actions => {
          //   actions.forEach(async stackName => {
          //     if (stacksAlreadyDeleting.includes(stackName)) return;

          //     console.log(`Deleting stack ${stackName}`);

          //     stacksAlreadyDeleting.push(stackName);

          //     const deleteStackInput: DeleteStackCommandInput = {
          //       StackName: stackName,
          //     }

          //     try {
          //       console.log('Deleting....');

          //       const stackReturn = await stackClient.send(new DeleteStackCommand(deleteStackInput));

          //       console.log('Client done');

          //       console.log(stackReturn);

          //       // return stackReturn;
          //     } catch (e) {
          //       console.log("Something went wrong");
          //       console.log(e);
          //       throw new Error();
          //     }
          //   });
          // });
      });

    for (const stackToDelete of stacksToDelete) {
      console.log(`Deleting stack ${stackToDelete}`);

      const deleteStackCommandInput: DeleteStackCommandInput = {
        StackName: stackToDelete,
      };
      const deleteStacksCommand = new DeleteStackCommand(deleteStackCommandInput);
      await stackClient.send(deleteStacksCommand);

      const waitUntilStackDeleteInput: DescribeStacksCommandInput = {
        StackName: stacksToDelete[0],
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
      } while(stackDeleteResult.state === WaiterState.RETRY);
    }

    // const branchName = event.BranchName;
    // if (branchName === "master") {
    //   console.log("Not deleting anything because this is the master branch.");
    //   return;
    // }

    // const client = new CloudFormationClient({ region: "us-east-1" });
    // const lambdaStackName = `monster-week-${branchName}-Stack-Beta`;

    // // Delete the pipeline stack
    // const deletePipelineInput: DeleteStackCommandInput = {
    //   StackName: `monster-week-${branchName}`,
    // };
    // let deletePipelineCommand = new DeleteStackCommand(deletePipelineInput);

    // console.log("attempt #1 to delete the stack");
    // // Attempt to delete the edge stack
    // let deleteDeployInput: DeleteStackCommandInput = {
    //   StackName: lambdaStackName,
    //   ClientRequestToken: id,
    // };
    // let deleteDeployCommand = new DeleteStackCommand(deleteDeployInput);

    // await Promise.all([
    //   client.send(deletePipelineCommand),
    //   client.send(deleteDeployCommand),
    // ]);

    // const waitUntilStackDeleteInput: DescribeStacksCommandInput = {
    //   StackName: lambdaStackName,
    // };
    // const waiterConfig: WaiterConfiguration<CloudFormationClient> = {
    //   client,
    //   maxWaitTime: 300,
    // };

    // console.log("awaiting stack deletion");
    // try {
    //   await waitUntilStackDeleteComplete(
    //     waiterConfig,
    //     waitUntilStackDeleteInput
    //   );

    //   console.log("Stack deleted");
    // } catch (e) {
    //   console.log("Stack did not delete. Trying again.", e);
    //   // Attempt the delete again with the lambda@edge resource ignored.
    //   deleteDeployInput = {
    //     StackName: lambdaStackName,
    //     RetainResources: ["LambdaEdge"],
    //     ClientRequestToken: id,
    //   };
    //   deleteDeployCommand = new DeleteStackCommand(deleteDeployInput);
    //   await client.send(deleteDeployCommand);
    // }

    // console.log("All stacks deleted. Good bye.");

    // return;
  } catch (e) {
    console.log("It broke", e);
    throw e;
  }
};
