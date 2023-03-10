import { PipelineDeclaration } from '@aws-sdk/client-codepipeline';
import { deleteStacks} from './delete';
import { deleteEdgeLambda } from './lambda';
import { getPipeline } from './pipeline';

export const handler = async (event: { BranchName: string }): Promise<any> => {
  console.log("event", event);

  const branchName = event.BranchName;

  const pipeline = await getPipeline(branchName);

  if (!pipeline) throw new Error(`No pipeline found for branch ${branchName}`)

  const promises = await Promise.allSettled([
    deleteStacks(pipeline as PipelineDeclaration),
    deleteEdgeLambda(),
  ]);

  const rejectedPromises =
    promises.filter((promise: PromiseSettledResult<void>) => promise.status === 'rejected') as Array<PromiseRejectedResult>;

  if (rejectedPromises.length > 0) {
    console.log(rejectedPromises.map((promise: PromiseRejectedResult) => promise.reason).join(" | "));
    throw new Error("Something didn't delete correctly.");
  }
}