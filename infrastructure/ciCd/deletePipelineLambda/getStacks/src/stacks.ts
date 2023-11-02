import { CloudFormationClient, ListStacksCommand, ListStacksCommandInput } from '@aws-sdk/client-cloudformation';
import { Stack } from './models';

export const getStacks = async (stackprefix: string): Promise<Array<Stack>> => {
    const stackClient = new CloudFormationClient({ region: "us-east-1" });
    const input: ListStacksCommandInput = {
        StackStatusFilter: [
            "CREATE_COMPLETE",
            "UPDATE_COMPLETE",
            "DELETE_FAILED"
        ]
    };
    let newxtToken = undefined;
    const stacks = [];

    do {
        input.NextToken = newxtToken;
        const command = new ListStacksCommand(input);
        const response = await stackClient.send(command);
        const stacksToAdd = response
            .StackSummaries
            ?.filter(stack => stack.StackName?.startsWith(stackprefix) && !stack.ParentId)
            .map(stack => ({ name: stack.StackName, status: stack.StackStatus }));
        if (stacksToAdd) stacks.push(...stacksToAdd);

        newxtToken = response.NextToken;
    } while (newxtToken);

    return stacks;
}