const {
  nativeStartTokioRuntime,
  nativeShutdownTokioRuntime,
  nativeStartServer,
  nativeSendFile,
  nativeGenRefId,
  nativeGetFileMeta
} = require('./native.node');

export type ServerConfig = {
  port: number;
  receiveFilesDir: string;
  onStart: () => void;
  onReceiveFileStart: (refId: string, from: { ip: string; port: number }, file: { name: string; size: number }) => void;
  onReceiveFileProgress: (refId: string, progress: number) => void;
  onReceiveFileComplete: (refId: string, outputPath: string) => void;
  onReceiveFileError: (refId: string, msg: string) => void;
  onServerError: (msg: string) => void;
};

export type SendFileConfig = {
  refId: string;
  ip: string;
  port: number;
  filePath: string;
  onSendFileStart: (refId: string) => void;
  onSendFileProgress: (refId: string, progress: number) => void;
  onSendFileComplete: (refId: string) => void;
  onSendFileError: (refId: string, msg: string) => void;
};

export function startTokioRuntime() {
  nativeStartTokioRuntime();
}

export function shutdownTokioRuntime() {
  nativeShutdownTokioRuntime();
}

export function startServer(config: ServerConfig) {
  nativeStartServer(config);
}

export function sendFile(config: SendFileConfig) {
  nativeSendFile(config);
}

export function genRefId(): string {
  return nativeGenRefId();
}

export function getFileMeta(path: string): { name: string; size: number } {
  return nativeGetFileMeta(path);
}
