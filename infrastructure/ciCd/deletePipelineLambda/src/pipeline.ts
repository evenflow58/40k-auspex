import { GetPipelineCommand, GetPipelineCommandInput, CodePipelineClient, DeletePipelineCommand, DeleteCustomActionTypeCommandInput, DeletePipelineCommandInput, PipelineDeclaration } from "@aws-sdk/client-codepipeline"

export const getPipeline = async (branchName: string): Promise<PipelineDeclaration | undefined> => {
    const client = new CodePipelineClient({ region: "us-east-1" });
    const getPipelineInput: GetPipelineCommandInput = {
        name: `monster-week-${branchName}`
    };
    const command = new GetPipelineCommand(getPipelineInput);
    const { pipeline } = await client.send(command);

    return pipeline;
}