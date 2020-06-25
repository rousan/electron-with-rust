export type ReceiveFileStatus =
  | { type: 'progress'; progress: number }
  | { type: 'complete'; outputPath: string }
  | { type: 'error'; msg: string };

export type SendFileStatus =
  | { type: 'connecting' }
  | { type: 'progress'; progress: number }
  | { type: 'complete' }
  | { type: 'error'; msg: string };

export type FileStatus =
  | { type: 'connecting'; ip: string; port: number }
  | { type: 'progress'; progress: number }
  | { type: 'complete'; filePath: string }
  | { type: 'error'; msg: string };
