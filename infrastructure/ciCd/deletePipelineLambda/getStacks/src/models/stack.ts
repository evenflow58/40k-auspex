import { StackStatus } from "@aws-sdk/client-cloudformation";

export class Stack {
    name: string;
    status: StackStatus;
}