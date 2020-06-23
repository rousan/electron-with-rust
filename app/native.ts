import { promisify } from 'util';

const {
  nativeFetchUrl,
  nativeStartRuntime,
  nativeShutdownRuntime,
  nativeAsyncTask,
  nativeFooTask
} = require('./native.node');

export async function fetchUrl(url: string): Promise<{ code: number; data: any }> {
  return promisify(nativeFetchUrl)(url);
}

export function startTokioRuntime() {
  nativeStartRuntime();
}

export function shutdownTokioRuntime() {
  nativeShutdownRuntime();
}

export function asyncTask() {
  nativeAsyncTask((a: string, b: number) => {
    console.log(a, b);
  });
}

export function fooTask(cb: any) {
  nativeFooTask(cb);
}
