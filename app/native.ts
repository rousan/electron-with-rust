const {
  nativeStartTokioRuntime,
  nativeShutdownTokioRuntime,
  nativeStartServer,
  nativeSendFile
} = require('./native.node');

export type ServerConfig = {
  port: number;
  onStart: () => void;
  onFileReceiveStart: (refId: string, from: { ip: string; port: number }, file: { name: string; size: number }) => void;
  onFileReceiveProgress: (refId: string, progress: number) => void;
  onFileReceiveComplete: (refId: string, outputPath: string) => void;
  onFileReceiveError: (refId: string, msg: string) => void;
  onServerError: (msg: string) => void;
};

export type SendFileConfig = {
  ip: string;
  port: number;
  filePath: string;
  onFileSendStart: (refId: string, file: { name: string; size: number }) => void;
  onFileSendProgress: (refId: string, progress: number) => void;
  onFileSendComplete: (refId: string) => void;
  onFileSendError: (refId: string, msg: string) => void;
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
