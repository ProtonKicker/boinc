export interface BoincTask {
  name?: string;
  wuName?: string;
  projectUrl?: string;
  state?: number;
  readyToReport?: boolean;
  gotServerAck?: boolean;
  receivedTime?: number;
  reportDeadline?: number;
  activeTask?: boolean;
  activeTaskState?: number;
  fractionDone?: number;
  elapsedTime?: number;
  estimatedCpuTimeRemaining?: number;
}

export interface BoincRpcStatus {
  connection: string;
  authorized?: boolean;
  error?: string;
}

