import { app, BrowserWindow } from 'electron';
import installExtension, { REDUX_DEVTOOLS, REACT_DEVELOPER_TOOLS } from 'electron-devtools-installer';
import electronDebug from 'electron-debug';
import { isProd, isDev } from './utils';

let mainWindow: BrowserWindow | null = null;

(async () => {
  if (isProd()) {
    require('source-map-support').install();
  }

  if (isDev() || process.env.DEBUG_PROD === 'true') {
    electronDebug();
  }

  await app.whenReady();

  if (isDev() || process.env.DEBUG_PROD === 'true') {
    await installExtension([REDUX_DEVTOOLS, REACT_DEVELOPER_TOOLS], !!process.env.UPGRADE_EXTENSIONS);
  }

  await createMainWindow();

  app.on('window-all-closed', () => {
    // Respect the OSX convention of having the application in memory even
    // after all windows have been closed.
    if (process.platform !== 'darwin') {
      app.quit();
    }
  });

  app.on('activate', async () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      await createMainWindow();
    }
  });
})().catch(err => {
  console.error(err);
});

async function createMainWindow() {
  mainWindow = new BrowserWindow({
    show: false,
    width: 550,
    height: 700,
    webPreferences: {
      nodeIntegration: true
    },
    resizable: false,
    center: true
  });

  if (isProd()) {
    await mainWindow.loadURL(`file://${__dirname}/app.prod.html`);
  } else {
    await mainWindow.loadURL(`file://${__dirname}/app.dev.html`);
  }

  mainWindow.show();
  mainWindow.focus();

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}
