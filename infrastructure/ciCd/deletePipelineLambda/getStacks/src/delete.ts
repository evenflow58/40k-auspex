import {
    CloudFormationClient,
    DeleteStackCommand,
    DeleteStackCommandInput,
    waitUntilStackDeleteComplete,
    DescribeStacksCommandInput,
    DescribeStackResourcesCommand,
    DescribeStackResourcesCommandInput,
    DeleteStackCommandOutput,
    ResourceStatus,
} from "@aws-sdk/client-cloudformation";
import { PipelineDeclaration } from "@aws-sdk/client-codepipeline";
import { WaiterConfiguration } from "@aws-sdk/util-waiter";

export const deleteStacks = async (pipeline: PipelineDeclaration) => {
    const stacksToDelete = getStacks(pipeline);

    if (stacksToDelete.length === 0) {
        console.log("No stacks to delete");
        return;
    }

    for (const stacks of stacksToDelete) {
        const deletes = await Promise.allSettled([
            ...stacks.map((stack: string) => fullStackDelete(stack)),
            deleteStack(pipeline.name as string),
        ]);

        if (deletes.find(result => result.status === "rejected")) throw new Error("Unable to delete stack");
    }
}

const getStacks = (pipeline: PipelineDeclaration): Array<Array<string>> => {
    if (!pipeline?.stages) {
        console.log("Empty pipeline.")
        return [];
    }

    return pipeline
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
                        (actionAcc[action.runOrder] as Array<string>).push(action.configuration?.StackName)
                    }

                    return actionAcc;
                }, []);

            if (actions.length > 0) stageAcc = stageAcc.concat(actions);

            return stageAcc;
        }, [])
        .filter(stack => stack)
        .reverse();
}

const deleteStack = (stackName: string, requestIds: Array<string> | undefined = undefined): Promise<DeleteStackCommandOutput> => {
    const stackClient = new CloudFormationClient({ region: "us-east-1" });
    const deleteStackCommandInput: DeleteStackCommandInput = {
        StackName: stackName,
        RetainResources: requestIds
    };

    return stackClient.send(new DeleteStackCommand(deleteStackCommandInput));
}

const fullStackDelete = async (stackName: string) => {
    console.log(`Deleting stack ${stackName}`);

    const stackClient = new CloudFormationClient({ region: "us-east-1" });

    await deleteStack(stackName);

    try {
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

        return
    } catch (e) {
        console.log("Something didn't delete. Delete everything else about the stack");

        const describeStackResourcesCommandInput: DescribeStackResourcesCommandInput = {
            StackName: stackName
        }
        const describeStackResourcessCommand = new DescribeStackResourcesCommand(describeStackResourcesCommandInput);

        const response = await stackClient.send(describeStackResourcessCommand);

        const idsToSkip = response.StackResources
            ?.filter(stack => stack.ResourceStatus === ResourceStatus.DELETE_FAILED)
            ?.map(stack => stack.LogicalResourceId);

        await deleteStack(stackName, idsToSkip as Array<string>);

        console.log(`stack: ${stackName}`, JSON.stringify(response));
    }
}