import { getStacks } from './stacks';

export const handler = async (event: { prefix: string }): Promise<any> => {
  console.log("event", event);
  const prefix = event.prefix;

  const stacks = await getStacks(prefix);

  // Finish
  if (stacks.length === 0) return { result: 'DONE' };

  // Send to delete
  return { result: 'DELETE', data: stacks.map(({ name }) => name), prefix };
}