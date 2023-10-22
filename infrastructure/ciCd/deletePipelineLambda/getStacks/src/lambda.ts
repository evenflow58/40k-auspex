import { DeleteFunctionCommand, DeleteFunctionCommandInput, FunctionConfiguration, LambdaClient, ListFunctionsCommand, ListFunctionsCommandInput } from "@aws-sdk/client-lambda"

export const deleteEdgeLambda = async () => {
    await Promise.allSettled(
        (await getEdgeLambda()).map((lambda: FunctionConfiguration) => {
            const client = new LambdaClient({ region: "us-east-1" });
            const deleteInput: DeleteFunctionCommandInput = {
                FunctionName: lambda.FunctionName,
            };
            const deleteCommand = new DeleteFunctionCommand(deleteInput);
            return client.send(deleteCommand);
        }));
}

const getEdgeLambda = async () => {
    const client = new LambdaClient({ region: "us-east-1" });

    const lambda = new Array<FunctionConfiguration>();
    let nextMarker;

    do {
        const listInput: ListFunctionsCommandInput = {
            MaxItems: 50
        };
        const listCommand = new ListFunctionsCommand(listInput)
        const result = await client.send(listCommand);

        if (result.Functions) lambda.push(...result.Functions);
        nextMarker = result.NextMarker;
    } while (nextMarker);

    return lambda.filter((l: FunctionConfiguration) => l.FunctionName?.includes("LambdaEdge"));
}